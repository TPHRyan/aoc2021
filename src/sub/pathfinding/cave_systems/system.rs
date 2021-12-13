mod connections;
mod nodes;

use std::collections::HashMap;
use std::error::Error as ErrorTrait;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::{Error, Result};
use connections::NodeConnections;
use nodes::*;

pub struct CaveSystem {
    nodes: Vec<CaveNode>,
    nodes_by_name: HashMap<String, usize>,
    connections: NodeConnections,
}

impl CaveSystem {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            nodes_by_name: HashMap::new(),
            connections: NodeConnections::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) -> usize {
        match self.nodes_by_name.get(node_name) {
            Some(node_id) => *node_id,
            None => {
                let new_node = match CaveNode::from_str(node_name) {
                    Ok(n) => n,
                    Err(e) => panic!("{}", e),
                };
                let new_node_id = self.nodes.len();
                self.nodes.push(new_node);
                self.nodes_by_name
                    .insert(String::from(node_name), new_node_id);
                new_node_id
            }
        }
    }

    pub fn add_connection(&mut self, id_a: usize, id_b: usize) {
        self.connections.connect_nodes(id_a, id_b)
    }
}

impl FromStr for CaveSystem {
    type Err = Box<dyn ErrorTrait>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut cave_system = CaveSystem::new();
        for line in s.lines() {
            let [a, b] = parse_cave_names(line)?;
            let a_id = cave_system.add_node(a);
            let b_id = cave_system.add_node(b);
            cave_system.add_connection(a_id, b_id);
        }
        Ok(cave_system)
    }
}

fn parse_cave_names(s: &str) -> Result<[&str; 2]> {
    match s.split("-").take(2).collect::<Vec<&str>>()[..] {
        [a, b] => Ok([a, b]),
        _ => Err(Box::new(Error::new(&format!(
            "Could not find two cave names in string {}!",
            s
        )))),
    }
}

impl CaveSystem {
    pub fn find_paths(&self, from_name: &str, to_name: &str) -> Vec<Vec<&str>> {
        let from = self.get_existing_node_id_by_name(from_name);
        let to = self.get_existing_node_id_by_name(to_name);
        let visited_from = VisitedCaves::new();
        self.find_paths_from(from, to, vec![], visited_from)
    }

    fn find_paths_from(
        &self,
        from: usize,
        to: usize,
        visited_small: Vec<usize>,
        visited_from: VisitedCaves,
    ) -> Vec<Vec<&str>> {
        let empty = vec![];
        if from > self.nodes.len() || to > self.nodes.len() {
            return empty;
        }
        match self.connections.get_for(from) {
            Some(connections) => {
                let from_node = &self.nodes[from];
                let mut found_paths = vec![];
                for next_id in connections {
                    let next_node = &self.nodes[*next_id];
                    let next_type = next_node.cave_type();
                    if *next_id == to {
                        found_paths.push(vec![next_node.name(), from_node.name()]);
                        continue;
                    }
                    match next_type {
                        CaveType::Start | CaveType::End => {
                            continue;
                        }
                        _ => {
                            let mut new_visited_small = visited_small.clone();
                            let mut new_visited_from = visited_from.clone();
                            if let CaveType::Small = next_type {
                                if visited_small.contains(next_id)
                                    || visited_from.have_already_visited(*next_id, from)
                                {
                                    continue;
                                }
                                new_visited_small.push(*next_id);
                            }
                            new_visited_from.visit_from(from, *next_id);
                            let mut next_paths: Vec<Vec<&str>> = self
                                .find_paths_from(*next_id, to, new_visited_small, new_visited_from)
                                .into_iter()
                                .map(|mut path| {
                                    path.push(from_node.name());
                                    path
                                })
                                .collect();
                            found_paths.append(&mut next_paths);
                        }
                    }
                }
                found_paths
            }
            None => empty,
        }
    }

    fn get_existing_node_id_by_name(&self, name: &str) -> usize {
        *self
            .nodes_by_name
            .get(name)
            .expect(&format!("Expected node \"{}\" to exist!", name))
    }
}

impl Debug for CaveSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.nodes.len() {
            if let Some(connections) = self.connections.get_for(i) {
                for other_id in connections {
                    writeln!(
                        f,
                        "{} -> {}",
                        self.nodes[i].name(),
                        self.nodes[*other_id].name()
                    )?;
                }
            }
        }
        writeln!(f)
    }
}

struct VisitedCaves {
    visited_from: HashMap<usize, HashMap<usize, ()>>,
}

impl VisitedCaves {
    pub fn new() -> Self {
        Self {
            visited_from: HashMap::new(),
        }
    }

    pub fn have_already_visited(&self, to_id: usize, from_id: usize) -> bool {
        match self.visited_from.get(&from_id) {
            Some(from_map) => from_map.contains_key(&to_id),
            None => false,
        }
    }

    pub fn visit_from(&mut self, from_id: usize, to_id: usize) {
        match self.visited_from.get_mut(&from_id) {
            Some(from_map) => {
                from_map.insert(to_id, ());
            }
            None => {
                self.visited_from.insert(from_id, HashMap::new());
                self.visited_from
                    .get_mut(&from_id)
                    .unwrap()
                    .insert(to_id, ());
            }
        }
    }
}

impl Clone for VisitedCaves {
    fn clone(&self) -> Self {
        let mut visited_from = HashMap::new();
        for (from, from_map) in self.visited_from.iter() {
            visited_from.insert(*from, from_map.clone());
        }
        Self { visited_from }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";

    #[test]
    fn can_parse_example_data() -> Result<()> {
        let cave_system = CaveSystem::from_str(TEST_STR)?;
        assert_eq!(10, cave_system.nodes.len());
        assert_eq!(18, cave_system.connections.len());
        Ok(())
    }

    #[test]
    fn can_find_paths_for_minimal_system() -> Result<()> {
        let test_str = "df-start\nzm-end\nstart-zm\nend-df";
        let cave_system = CaveSystem::from_str(test_str)?;
        let expected_paths = vec!["start,df,end", "start,zm,end"];
        let paths = cave_system.find_paths("start", "end");
        for (mut path, expected_path) in paths.into_iter().zip(expected_paths) {
            path.reverse();
            assert_eq!(expected_path, path.join(","));
        }
        Ok(())
    }

    #[test]
    fn can_find_correct_number_of_paths_through_small_caves() -> Result<()> {
        let test_str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let cave_system = CaveSystem::from_str(test_str)?;
        let paths = cave_system.find_paths("start", "end");
        assert_eq!(10, paths.len());
        Ok(())
    }
}
