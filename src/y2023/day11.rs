use crate::prelude::*;

use self::parser::parse_graph;

day!(11);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let mut graph = whatever!(parse_graph(&input), "Failed to parse input");
    
    let mut p1graph = graph.clone();
    p1graph.expand(1);
    let part1 = solve(&p1graph);
    println!("Part 1: {part1}");

    graph.expand(999_999);
    let part2 = solve(&graph);
    println!("Part 2: {part2}");

    Ok(())
}

fn solve(graph: &Graph) -> u64 {
    let mut total = 0u64;

    for (i, start) in graph.nodes.iter().enumerate() {
        for end in &graph.nodes[i+1..] {
            let dist = start.dist(end);
            total += dist as u64;
        }
    }

    total
}

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn dist(&self, other: &Self) -> usize {
        let xdist = self.x.abs_diff(other.x);
        let ydist = self.y.abs_diff(other.y);
        xdist + ydist
    }
}

#[derive(Debug, Clone)]
struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<Coord>,
}

impl Graph {
    pub fn expand(&mut self, amount: usize) {
        let old_nodes = self.nodes.clone();

        // Expand rows
        for row in 0..self.height {
            let is_empty = !old_nodes.iter().any(|n| n.y == row);
            if is_empty {
                self.height += amount;
                for (old, node) in old_nodes.iter().zip(&mut self.nodes) {
                    if old.y > row {
                        node.y += amount;
                    }
                }
            }
        }

        // Expand columns
        for col in 0..self.width {
            let is_empty = !old_nodes.iter().any(|n| n.x == col);
            if is_empty {
                self.width += amount;
                for (old, node) in old_nodes.iter().zip(&mut self.nodes) {
                    if old.x > col {
                        node.x += amount;
                    }
                }
            }
        }
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::{eof, value},
        Finish, IResult,
    };

    use crate::utils::NomFail;

    use super::{Coord, Graph};

    pub fn parse_graph(input: &[u8]) -> Result<Graph, NomFail> {
        match parse_graph_(input).finish() {
            Ok((_, g)) => Ok(g),
            Err(e) => Err(e.into()),
        }
    }

    fn parse_graph_(mut input: &[u8]) -> IResult<&[u8], Graph> {
        let mut nodes = Vec::new();
        let mut tile = alt((value(false, tag(b".")), value(true, tag(b"#"))));
        let line_ending = line_ending::<_, nom::error::Error<_>>;
        let mut width = None;
        let mut height = None;

        for y in 0usize.. {
            if input.is_empty() {
                height = Some(y);
                break;
            }

            for x in 0usize.. {
                match width {
                    Some(w) => {
                        if x >= w {
                            (input, _) = alt((eof, line_ending))(input)?;
                            break;
                        }
                    }
                    None => {
                        if let Ok((inp, _)) = line_ending(input) {
                            input = inp;
                            width = Some(x);
                            break;
                        }
                    }
                };

                let (inp, gal) = tile(input)?;
                input = inp;
                if gal {
                    nodes.push(Coord { x, y });
                }
            }
        }

        eof(input)?;
        let width = width.unwrap_or_default();
        let height = height.unwrap_or_default();

        Ok((
            input,
            Graph {
                width,
                height,
                nodes,
            },
        ))
    }
}
