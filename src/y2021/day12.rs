
use crate::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

day!(12);


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum NodeType {
    Start,
    End,
    Big,
    Small
}

impl NodeType {
    pub fn of_str(s: &str) -> Self {
        match s {
            "start" => NodeType::Start,
            "end"   => NodeType::End,
            s if s.chars().next().unwrap().is_ascii_uppercase()
                    => NodeType::Big,
            _       => NodeType::Small
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    ntype: NodeType,
    neighbors: Vec<usize>
}

impl From<NodeType> for Node {
    fn from(ntype: NodeType) -> Self {
        Node {
            ntype,
            neighbors: Vec::new()
        }
    }
}

#[derive(Clone, Debug)]
struct CaveSystem {
    idx_map: HashMap<String, usize>,
    nodes: Vec<Node>
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        CaveSystem {
            idx_map: [("start".to_string(), 0), ("end".to_string(), 1)].into(),
            nodes: vec![NodeType::Start.into(), NodeType::End.into()]
        }
    }

    pub fn get(&self, key: &str) -> Option<&Node> {
        self.idx_map
            .get(key)
            .and_then(|i| self.nodes.get(*i))
    }

    pub fn get_idx(&mut self, key: String, ntype: NodeType) -> usize {
        *self.idx_map
            .entry(key)
            .or_insert_with(|| {
                let idx = self.nodes.len();
                self.nodes.insert(idx, ntype.into());
                idx
            })
    }

    fn add_neighbors(&mut self, a: usize, b: usize) {
        self.nodes[a].neighbors.push(b);
        self.nodes[b].neighbors.push(a);
    }

    fn is_valid_move(&self, past: &[(usize, &[usize])], new: usize, second: bool) -> bool {
        let last = match past.last() {
            Some(_) if new == 0 => return false,
            None => return new == 0,
            Some(&(l, _)) => &self[l],
        };

        if !last.neighbors.contains(&new) {
            return false;
        }

        if self[new].ntype == NodeType::Small {
            let exists = past.iter().any(|&(i, _)| i == new);
            if exists {
                if second {
                    let any2 =
                        past.iter()
                            .filter_map(|&(i,_)| (self[i].ntype == NodeType::Small).then(|| i))
                            .counts()
                            .values()
                            .any(|&v| v > 1);
                    if any2 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_paths(&self, second: bool) -> u64 {
        let mut count = 0;
        let mut stack = vec![(0, &*self[0].neighbors)];

        loop {
            let cur = match stack.last_mut() {
                Some((_, cur)) => cur,
                None => return count
            };

            let (&first, rest) = match cur.split_first() {
                Some(res) => res,
                None => {
                    stack.pop();
                    continue;
                }
            };

            *cur = rest;

            if self.is_valid_move(&stack, first, second) {
                let fnode = &self[first];
                if fnode.ntype == NodeType::End {
                    count += 1;
                } else {
                    stack.push((first, &*fnode.neighbors));
                }
            }
        }
    }
}

impl<'idx> Index<&'idx str> for CaveSystem {
    type Output = Node;

    fn index(&self, idx: &'idx str) -> &Node {
        self.get(idx).unwrap()
    }
}

impl Index<usize> for CaveSystem {
    type Output = Node;

    fn index(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }
}

impl FromIterator<String> for CaveSystem {
    fn from_iter<T: IntoIterator<Item=String>>(iter: T) -> CaveSystem {
        let mut caves = CaveSystem::new();
        for line in iter {
            let (cave1, cave2) =
                line.split('-')
                    .map(|s| caves.get_idx(s.to_string(), NodeType::of_str(s)))
                    .collect_tuple()
                    .unwrap();
            caves.add_neighbors(cave1, cave2);
        }
        caves
    }
}

pub fn run() -> GenResult {
    let caves: CaveSystem = load_input(DAY)?.lines().try_collect()?;

    let part1 = caves.count_paths(false);
    println!("Part 1: {}", part1);

    let part2 = caves.count_paths(true);
    println!("Part 2: {}", part2);

    Ok(())
}