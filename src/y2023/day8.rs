use std::collections::HashMap;

use crate::{prelude::*, utils::NomFail};

day!(8);

fn part1(graph: &Graph, dirs: &[Dir]) -> Result<u64, Whatever> {
    let start = graph.labels[b"AAA"];

    let mut count = 0u64;
    let mut idx = start;
    let mut dir_iter = dirs.iter().copied().cycle();
    loop {
        count += 1;
        let dir = dir_iter.next().expect("Ran out of steps");
        let Some((fnode, fidx)) = graph.follow(idx, dir) else {
            whatever!("Failed to follow node");
        };

        if fnode == *b"ZZZ" {
            return Ok(count);
        }

        idx = fidx;
    }
}

fn part2(graph: &Graph, dirs: &[Dir]) -> Result<u64, Whatever> {
    let mut nodes: Vec<usize> = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.label[2] == b'A')
        .map(|(i, _)| i)
        .collect();

    let mut count = 0u64;
    let mut dir_iter = dirs.iter().copied().cycle();
    let mut lcm = 1u64;
    while !nodes.is_empty() {
        count += 1;
        let dir = dir_iter.next().expect("Ran out of steps");
        nodes.retain_mut(|idx| {
            let (fnode, fidx) = graph.follow(*idx, dir)
                .expect("Failed to follow node");

            *idx = fidx;

            if fnode[2] == b'Z' {
                lcm = num::integer::lcm(lcm, count);
                false
            } else {
                true
            }
        });
    }

    Ok(lcm)
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let (_, (dirs, graph)) = whatever!(parser::parse_all(&input)
        .finish()
        .map_err(NomFail::from),
        "Failed to parse input");

    let part1 = whatever!(part1(&graph, &dirs), "Failed doing part 1");
    println!("Part 1: {part1}");

    let part2 = whatever!(part2(&graph, &dirs), "Failed doing part 2");
    println!("Part 2: {part2}");

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

type Label = [u8; 3];

#[derive(Debug)]
struct Node<T> {
    label: Label,
    dirs: Option<(T, T)>,
}

#[derive(Debug, Default)]
struct Graph {
    labels: HashMap<Label, usize>,
    nodes: Vec<Node<usize>>,
}

impl Graph {
    pub fn follow(&self, idx: usize, dir: Dir) -> Option<(Label, usize)> {
        let (left, right) = self.nodes.get(idx)?.dirs?;
        let fidx = match dir {
            Dir::Left => left,
            Dir::Right => right,
        };
        Some((self.nodes[fidx].label, fidx))
    }

    pub fn add_label(&mut self, label: Label) -> usize {
        *self.labels.entry(label)
            .or_insert_with(|| {
                let idx = self.nodes.len();
                let node = Node { label, dirs: None };
                self.nodes.push(node);
                idx
            })
    }
}

impl FromIterator<Node<Label>> for Graph {
    fn from_iter<T: IntoIterator<Item = Node<Label>>>(iter: T) -> Self {
        let mut graph = Graph::default();
        for item in iter {
            let Node { label, dirs: ldirs } = item;
            let idx = graph.add_label(label);
            let dirs = ldirs.map(|(left, right)| 
                (graph.add_label(left), graph.add_label(right))
            );
            graph.nodes[idx].dirs = dirs;
        }
        graph
    }
}

mod parser {
    use super::{Graph, Label, Node, Dir};
    use crate::utils::{parser::{many_array, self}, eof_iterator};
    use nom::{
        character::complete::{char, space1, multispace1},
        sequence::{tuple, separated_pair},
        IResult, combinator::{map, value}, multi::many1, branch::alt,
    };

    pub fn parse_all(input: &[u8]) -> IResult<&[u8], (Vec<Dir>, Graph)> {
        separated_pair(parse_dirs, multispace1, parse_graph)(input)
    }

    fn parse_label(input: &[u8]) -> IResult<&[u8], Label> {
        many_array(nom::number::complete::u8)(input)
    }

    fn parse_dirs(input: &[u8]) -> IResult<&[u8], Vec<Dir>> {
        let pdir = alt((
            value(Dir::Left, char('L')),
            value(Dir::Right, char('R')),
        ));

        many1(pdir)(input)
    }

    fn parse_graph(mut input: &[u8]) -> IResult<&[u8], Graph> {
        let pline = parser::line(tuple((
            parse_label,
            space1,
            char('='),
            space1,
            char('('),
            parse_label,
            char(','),
            space1,
            parse_label,
            char(')'),
        )));

        let pline = map(pline, |(n, _, _, _, _, l, _, _, r, _)| Node {
            label: n,
            dirs: Some((l, r)),
        });

        let mut iter = eof_iterator(input, pline);
        let graph: Graph = iter.collect();
        (input, ()) = iter.finish()?;

        Ok((input, graph))
    }
}
