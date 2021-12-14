use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use std::fmt;
use std::fmt::Debug;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph:\n")?;

        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];

            for j in self.successors(i) {
                let dest = &self.nodes[j];
                write!(f, "\t{:?}->{:?}\n", node.cave, dest.cave)?
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct NodeData {
    cave: Cave,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn get_node(&self, cave: &Cave) -> Option<NodeIndex> {
        let (idx, _data) = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_idx, node_data)| &node_data.cave == cave)?;

        Some(idx)
    }

    fn get_node_data(&self, index: NodeIndex) -> Option<&NodeData> {
        self.nodes.get(index)
    }

    fn add_node(&mut self, cave: Cave) -> NodeIndex {
        match self.get_node(&cave) {
            Some(idx) => idx,
            None => {
                let index = self.nodes.len();
                self.nodes.push(NodeData {
                    cave,
                    first_outgoing_edge: None,
                });
                index
            }
        }
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    fn size(&self) -> usize {
        self.edges.iter().fold(0, |acc, node| acc.max(node.target)) + 1
    }
}

#[derive(Debug)]
struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Cave {
    START,
    END,
    SMALL(String),
    LARGE(String),
}

fn small_cave(s: &str) -> IResult<&str, Cave> {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let (rest, name) = take_while1(move |c| chars.contains(c))(s)?;

    let string_name = name.to_string();

    Ok((rest, Cave::SMALL(string_name)))
}

fn large_cave(s: &str) -> IResult<&str, Cave> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let (rest, name) = take_while1(move |c| chars.contains(c))(s)?;

    Ok((rest, Cave::LARGE(name.to_string().clone())))
}

fn start_cave(s: &str) -> IResult<&str, Cave> {
    let (rest, _name) = tag("start")(s)?;

    Ok((rest, Cave::START))
}

fn end_cave(s: &str) -> IResult<&str, Cave> {
    let (rest, _name) = tag("end")(s)?;

    Ok((rest, Cave::END))
}

fn cave(s: &str) -> IResult<&str, Cave> {
    let (res, cave) = alt((start_cave, end_cave, large_cave, small_cave))(s)?;
    Ok((res, cave))
}

fn connection(s: &str) -> IResult<&str, (Cave, Cave)> {
    let (rest, (start, _, end)) = tuple((cave, tag("-"), cave))(s)?;

    Ok((rest, (start, end)))
}

fn graph_from_data(data: String) -> Graph {
    let (_rest, caves) = separated_list1(line_ending, connection)(&data).unwrap();
    let mut graph = Graph::new();

    for (source, dest) in caves {
        let source_node = graph.add_node(source);
        let dest_node = graph.add_node(dest);
        graph.add_edge(source_node, dest_node);
        graph.add_edge(dest_node, source_node);
    }

    graph
}

pub fn part1(data: String) -> usize {
    let graph = graph_from_data(data);
    let mut small_counts: Vec<usize> = Vec::new();
    small_counts.resize(graph.size(), 0);

    count_paths(
        &graph,
        graph.get_node(&Cave::START).unwrap(),
        small_counts,
        1,
    )
}

pub fn part2(data: String) -> usize {
    let graph = graph_from_data(data);
    let mut small_counts: Vec<usize> = Vec::new();
    small_counts.resize(graph.size(), 0);

    count_paths(
        &graph,
        graph.get_node(&Cave::START).unwrap(),
        small_counts,
        2,
    )
}

fn count_paths(
    graph: &Graph,
    curr_pos: NodeIndex,
    mut small_counts: Vec<usize>,
    max_small_counts: usize,
) -> usize {
    let curr_node = graph.get_node_data(curr_pos).unwrap();

    if curr_node.cave == Cave::END {
        return 1;
    }

    let curr_node = graph.get_node_data(curr_pos).unwrap();
    let mut count = 0;

    if matches!(curr_node.cave, Cave::SMALL(_)) {
        small_counts[curr_pos] += 1;
    }

    for next_pos in graph.successors(curr_pos) {
        let next_node = graph.get_node_data(next_pos).unwrap();

        if next_node.cave == Cave::START {
            continue;
        }

        if matches!(next_node.cave, Cave::SMALL(_))
            && small_counts[next_pos] >= 1
            && small_counts.iter().any(|c| *c >= max_small_counts)
        {
            continue;
        }

        let new_count = count_paths(graph, next_pos, small_counts.clone(), max_small_counts);

        count += new_count;
    }

    count
}
