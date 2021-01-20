use std::fs::File;
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

    for _ in 0..iterations {
      let mut functions = vec![];
      for block in loop_body {
        if let Block::Function { function_type, .. } = block {
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

    dbg!(&function_iterations);

    let mut from = 0;
    let parallel_fors = function_iterations.into_iter().enumerate().map(|(fi, (function_names, function_iterations))| {
      let block = Block::ParallelFor {
        name: format!("{}_{}", name, fi),
        data_ins: data_ins.clone().map(|data_ins| {
          data_ins.into_iter().map(|mut data_in| {
            data_in.source = Some(format!("{}/{}", name, data_in.name));
            data_in
          }).collect()
        }),
        loop_counter: LoopCounter { from: from.to_string(), to: (from + function_iterations).to_string(), step: loop_counter.step.clone() },
        loop_body: loop_body.iter().enumerate().map(|(i, block)| {
          let mut block = block.clone();

          if let Block::Function { properties: Some(properties), .. } = &mut block {
            for property in properties.iter_mut() {

              if property.name == "resource" {
                property.value = format!("${{{}_url}}", function_names[i]);
              }
            }
          }

          block
        }).collect(),
        data_outs: data_outs.clone(), // TODO: map names
      };

      from += function_iterations;

      block
    }).collect();

    let data_ins = data_ins.clone().map(|data_ins| data_ins.into_iter().map(|mut data_in| {
      data_in.constraints = None;
      data_in
    }).collect());

    Ok(Block::Parallel {
      name: name.clone(),
      data_ins,
      parallel_body: vec![ParallellSection { section: parallel_fors }],
      data_outs: data_outs.clone(),
    })
  } else {
    Ok(block.clone())
  }
}

#[async_std::main]
async fn main() -> Result<()> {
  let file = File::open("../project/stock-fc.yml")?;

  let mut fc: FunctionChoreography = serde_yaml::from_reader(&file)?;
  // dbg!(&fc);

  fc.to_graph();

  let iterations = 20;
  let concurrency_limit = 2;

  for block in &mut fc.workflow_body {
    *block = schedule_parallel_for(block, iterations, concurrency_limit).await?;
  }

  dbg!(fc);

  Ok(())
}
