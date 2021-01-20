use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use anyhow::Result;

use dotenv_codegen::dotenv;

mod sql;
use sql::SqlClient;

mod afcl;
use afcl::{FunctionChoreography, Block, LoopCounter, ParallellSection};

async fn schedule_parallel_for(block: &Block, iterations: usize, concurrency_limit: usize) -> Result<Block> {
  let user = dotenv!("DB_USER");
  let url = dotenv!("URL");
  let database = dotenv!("DATABASE");
  let password = dotenv!("PASSWORD");
  let sql_client = SqlClient::new(user, password, url, database).await?;

  if let Block::ParallelFor { name, data_ins, data_outs, loop_counter, loop_body } = block {
    let mut est = HashMap::<String, (f64, f64)>::new();
    let mut concurrency_limits = HashMap::<String, usize>::new();
    let mut function_iterations = HashMap::<Vec<String>, usize>::new();
    let mut function_names = Vec::new();

    for _ in 0..iterations {
      let mut functions = vec![];
      for block in loop_body {
        if let Block::Function { name, function_type, .. } = block {
          if !function_names.contains(&name) {
            function_names.push(&name);
          }

          let mut function_deployments = sql_client.function_type_metadata(function_type).await?;
          function_deployments.sort_by_key(|d| ordered_float::OrderedFloat::<f64>(d.avg_rtt));

          for function in &function_deployments {
            if !est.contains_key(&function.name) {
              est.insert(function.name.to_owned(), (0.0, function.avg_rtt));
            }

            if !concurrency_limits.contains_key(&function.name) {
              concurrency_limits.insert(function.name.to_owned(), concurrency_limit);
            }
          }

          let mut est_functions: Vec<_> = est.iter_mut()
            .filter(|&(k, _)| function_deployments.iter().any(|d| &d.name == k) ).collect();

          est_functions.sort_by_key(|&(_, &mut (v, _))| ordered_float::OrderedFloat::<f64>(v));

          for (name, (e, rtt)) in est_functions {
            if let Some(limit) = concurrency_limits.get_mut(name) {
              if *limit == 0 {
                *e += *rtt;
                *limit = concurrency_limit;

                continue
              }

              *limit -= 1;
            }

            functions.push(name.to_owned());
            break;
          }
        }
      }

      let fi = function_iterations.get(&functions).unwrap_or(&0) + 1;
      function_iterations.insert(functions, fi);
    }

    let mut function_iterations = function_iterations.into_iter().map(|(k, v)| {
      (k[0].clone(), (k, v))
    }).collect::<Vec<_>>();
    function_iterations.sort_by_key(|&(_, (_, v))| v);
    function_iterations.reverse();

    let mut function_iterations2 = HashMap::<String, (Vec<String>, usize)>::new();

    for (k0, (k, v)) in function_iterations {
      if let Some((_, v2)) =function_iterations2.get_mut(&k0) {
        *v2 += v;
      } else {
        function_iterations2.insert(k0, (k, v));
      }
    }

    let function_iterations: HashMap::<Vec<String>, usize> = function_iterations2.into_iter().map(|(_, v)| v).collect();

    let mut from = 0;
    let parallel_fors: Vec<Block> = function_iterations.into_iter().enumerate().map(|(fi, (function_deployment_names, function_iterations))| {
      let new_name = format!("{}_{}", name, fi);
      let mut block = Block::ParallelFor {
        name: new_name.clone(),
        data_ins: data_ins.clone().map(|data_ins| {
          data_ins.into_iter().map(|mut data_in| {
            data_in.source = Some(format!("{}/{}", name, data_in.name));
            data_in
          }).collect()
        }),
        loop_counter: LoopCounter { from: from.to_string(), to: (from + function_iterations).to_string(), step: loop_counter.step.clone() },
        loop_body: loop_body.iter().enumerate().map(|(i, block)| {
          let mut block = block.clone();

          block.change_function_name(name, &new_name);
          for function_name in &function_names {
            block.change_function_name(function_name, &format!("{}_{}", function_name, fi));
          }

          if let Block::Function { data_ins, properties, .. } = &mut block {
            if let Some(properties) = properties {
              for property in properties.iter_mut() {

                if property.name == "resource" {
                  property.value = format!("${{{}_url}}", function_deployment_names[i]);
                }
              }
            }
          }

          block
        }).collect(),
        data_outs: data_outs.clone(), // TODO: map names
      };

      from += function_iterations;

      for function_name in &function_names {
        block.change_function_name(function_name, &format!("{}_{}", function_name, fi));
      }

      block
    }).collect();

    let data_ins = data_ins.clone().map(|data_ins| data_ins.into_iter().map(|mut data_in| {
      data_in.constraints = None;
      data_in
    }).collect());

    Ok(Block::Parallel {
      name: name.clone(),
      data_ins,
      parallel_body: parallel_fors.into_iter().map(|section| ParallellSection { section: vec![section] } ).collect(),
      data_outs: data_outs.clone(),
    })
  } else {
    Ok(block.clone())
  }
}

#[async_std::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 || args.len() > 2 {
    eprintln!("Invalid amount of arguments specified. The path to the FC is required as the only argument.");
    process::exit(1);
  }

  let file = File::open(&args[1])?;

  let mut fc: FunctionChoreography = serde_yaml::from_reader(&file)?;
  // dbg!(&fc);

  fc.to_graph();

  let iterations = 20;
  let concurrency_limit = 2;

  for block in &mut fc.workflow_body {
    *block = schedule_parallel_for(block, iterations, concurrency_limit).await?;
  }

  let yaml = fc.to_yaml()?;

  let file_path = Path::new(&args[1]);
  let file_name = file_path.file_stem().expect("Cannot read name of input file.");
  let output_path = file_path.with_file_name(&format!("{}-cfcl.yml", file_name.to_string_lossy()));
  let mut out_file = File::create(output_path)?;
  out_file.write_all(&yaml.into_bytes())?;
  Ok(())
}
