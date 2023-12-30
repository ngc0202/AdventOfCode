use std::{collections::{HashMap, HashSet}, io::Write};

use crate::prelude::*;
use rand::Rng;
use self::parser::parse_graph;

day!(25);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let graph = whatever!(parse_graph(&input), "Failed to parse input");
    
    let graph = loop {
        let mut graph = graph.clone();
        graph.fully_contract();
        if graph.edges.len() == 3 {
            break graph;
        }
    };

    let mut stdout = std::io::stdout().lock();
    for node in &graph.nodes {
        if !node.is_empty() {
            write!(&mut stdout, "{}: ", node.len()).unwrap();
            for l in node {
                stdout.write_all(l).unwrap();
                stdout.write_all(b" ").unwrap();
            }
            stdout.write_all(b"\n\n").unwrap();
        }
    } 

    println!("Links: {}", graph.edges.len());

    Ok(())
}

type Label = [u8; 3];

#[derive(Default, Debug, Clone)]
struct Graph {
    labels: HashMap<Label, usize>,
    nodes: Vec<HashSet<Label>>,
    edges: Vec<[usize; 2]>,
}

impl Graph {
    pub fn fully_contract(&mut self) {
        let Some(iters) = self.nodes.len().checked_sub(2) else {
            return;
        };

        let mut r = rand::thread_rng();

        for _ in 0..iters {
            let rand_edge = r.gen_range(0..self.edges.len());
            self.contract(rand_edge);
        }
    }

    pub fn insert_label(&mut self, label: Label) -> usize {
        *self.labels.entry(label)
            .or_insert_with(|| {
                let idx = self.nodes.len();
                self.nodes.push(HashSet::from([label]));
                idx
            })
    }

    pub fn contract(&mut self, n: usize) {
        // Remove self edge
        let [left, right] = self.edges.swap_remove(n);
        
        // Merge nodes
        let [lnode, rnode] = get_two_mut(&mut self.nodes, left, right);
        lnode.extend(rnode.drain());

        // Update right's adjacencies to left's
        self.edges.retain_mut(|[l, r]| {
            if *l == right {
                *l = left;
            }

            if *r == right {
                *r = left;
            }

            *l != *r
        });
    }
}

fn get_two_mut<T>(slice: &mut [T], one: usize, two: usize) -> [&mut T; 2] {
    use std::cmp::Ordering::*;
    let (swap, one, two) = match one.cmp(&two) {
        Less => (false, one, two),
        Equal => panic!("Cannot get two of the same index"),
        Greater => (true, two, one),
    };

    let (left, right) = slice.split_at_mut(one+1);
    let val1 = left.last_mut().unwrap();
    let val2 = &mut right[two - one - 1];
    if swap {
        [val2, val1]
    } else {
        [val1, val2]
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, space1},
        combinator::{eof, verify},
        multi::fold_many1,
        sequence::{preceded, terminated},
        Finish, IResult,
    };

    use crate::utils::{parser::many_array, NomFail};

    use super::{Graph, Label};

    pub fn parse_graph(input: &[u8]) -> Result<Graph, NomFail> {
        match parse_graph_(input).finish() {
            Ok((_, g)) => Ok(g),
            Err(err) => Err(err.into()),
        }
    }

    fn parse_graph_(mut input: &[u8]) -> IResult<&[u8], Graph> {
        let mut graph = Graph::default();
        while !input.is_empty() {
            // Parse prefix
            let pre;
            (input, pre) = terminated(label, tag(":"))(input)?;

            // Insert prefix
            let pre = graph.insert_label(pre);

            // Parse adjacencies
            (input, ()) = fold_many1(
                preceded(space1, label),
                || (),
                |(), n| {
                    // Add label and adjacency
                    let right = graph.insert_label(n);
                    graph.edges.push([pre, right]);
                },
            )(input)?;
            (input, _) = alt((eof, line_ending))(input)?;
        }
        _ = eof(input)?;
        Ok((input, graph))
    }

    fn label(input: &[u8]) -> IResult<&[u8], Label> {
        many_array(verify(nom::number::complete::u8, |b| {
            (b'a'..=b'z').contains(b)
        }))(input)
    }
}
