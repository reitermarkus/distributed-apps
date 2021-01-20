use std::collections::HashMap;

use petgraph::{Graph, graph::NodeIndex};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionChoreography {
  pub name: String,
  pub data_ins: Option<Vec<DataIO>>,
  pub data_outs: Option<Vec<DataIO>>,
  pub workflow_body: Vec<Block>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Block {
  #[serde(rename_all = "camelCase")]
  Function {
    name: String,
    #[serde(rename = "type")]
    function_type: String,
    data_ins: Option<Vec<DataIO>>,
    data_outs: Option<Vec<DataIO>>,
    properties: Option<Vec<Constraint>>,
  },
  #[serde(rename_all = "camelCase")]
  ParallelFor {
    name: String,
    data_ins: Option<Vec<DataIO>>,
    data_outs: Option<Vec<DataIO>>,
    loop_counter: LoopCounter,
    loop_body: Vec<Block>,
  },
  #[serde(rename_all = "camelCase")]
  Parallel {
    name: String,
    data_ins: Option<Vec<DataIO>>,
    parallel_body: Vec<ParallellSection>,
    data_outs: Option<Vec<DataIO>>,
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCounter {
  pub from: String,
  pub to: String,
  pub step: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallellSection {
  pub section: Vec<Block>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataIO {
  name: String,
  #[serde(rename = "type")]
  data_type: String,
  source: Option<String>,
  passing: Option<bool>,
  constraints: Option<Vec<Constraint>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
  name: String,
  value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  name: String,
  value: String,
}

trait AddToGraph {
  fn add_to_graph<'a>(&'a self, graph: &mut Graph<&'a str, ()>, functions: &mut HashMap<&'a str, NodeIndex>);
}

impl AddToGraph for FunctionChoreography {
  fn add_to_graph<'a>(&'a self, graph: &mut Graph<&'a str, ()>, functions: &mut HashMap<&'a str, NodeIndex>) {
    let index = function_index(graph, functions, &self.name);

    for block in &self.workflow_body {
      block.add_to_graph(graph, functions);
    }
  }
}

fn function_index<'a>(graph: &mut Graph<&'a str, ()>, functions: &mut HashMap<&'a str, NodeIndex>, name: &'a str) -> NodeIndex {
  if let Some(i) = functions.get(name) {
    *i
  } else {
    let i = graph.add_node(name);
    functions.insert(name, i);
    i
  }
}

fn source_index<'a>(graph: &mut Graph<&'a str, ()>, functions: &mut HashMap<&'a str, NodeIndex>, source: Option<&'a str>) -> Option<NodeIndex> {
  if let Some(source) = &source {
    if source.contains("/") {
      if let Some(source) = source.splitn(2, "/").nth(0) {
        return Some(function_index(graph, functions, source))
      }
    }
  }

  None
}

impl AddToGraph for Block {
  fn add_to_graph<'a>(&'a self, graph: &mut Graph<&'a str, ()>, functions: &mut HashMap<&'a str, NodeIndex>) {
    match self {
      Self::Function { name, data_ins, data_outs, .. } => {
        let index = function_index(graph, functions, name);

        if let Some(data_ins) = data_ins {
          for data_in in data_ins {
            if let Some(source_index) = source_index(graph, functions, data_in.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }

        if let Some(data_outs) = data_outs {
          for data_out in data_outs {
            if let Some(source_index) = source_index(graph, functions, data_out.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }
      },
      Self::ParallelFor { name, loop_counter, loop_body, data_ins, data_outs, .. } => {
        let index = function_index(graph, functions, name);

        if let Some(source_index) = source_index(graph, functions, Some(&loop_counter.from)) {
          graph.add_edge(index, source_index, ());
        }

        if let Some(source_index) = source_index(graph, functions, Some(&loop_counter.to)) {
          graph.add_edge(index, source_index, ());
        }

        if let Some(source_index) = source_index(graph, functions, Some(&loop_counter.step)) {
          graph.add_edge(index, source_index, ());
        }

        if let Some(data_ins) = data_ins {
          for data_in in data_ins {
            if let Some(source_index) = source_index(graph, functions, data_in.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }

        for function in loop_body {
          function.add_to_graph(graph, functions);
        }

        if let Some(data_outs) = data_outs {
          for data_out in data_outs {
            if let Some(source_index) = source_index(graph, functions, data_out.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }
      },
      Self::Parallel { name, data_ins, data_outs, parallel_body, .. } => {
        let index = function_index(graph, functions, name);

        if let Some(data_ins) = data_ins {
          for data_in in data_ins {
            if let Some(source_index) = source_index(graph, functions, data_in.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }

        for section in parallel_body {
          for block in &section.section {
            block.add_to_graph(graph, functions);
          }
        }

        if let Some(data_outs) = data_outs {
          for data_out in data_outs {
            if let Some(source_index) = source_index(graph, functions, data_out.source.as_deref()) {
              graph.add_edge(index, source_index, ());
            }
          }
        }
      }
    }
  }
}

impl FunctionChoreography {
  pub fn to_graph(&self) {
    let mut graph = Graph::<&str, ()>::new();

    let mut functions = HashMap::<&str, NodeIndex>::new();

    self.add_to_graph(&mut graph, &mut functions);

    dbg!(&functions);
    dbg!(&graph);

    use petgraph::dot::{Dot, Config};

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
  }
}
