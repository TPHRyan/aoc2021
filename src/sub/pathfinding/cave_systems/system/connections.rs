use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeConnections {
    connections: HashMap<usize, Vec<usize>>,
}

impl NodeConnections {
    pub fn new() -> Self {
        NodeConnections {
            connections: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.connections
            .values()
            .fold(0, |acc, node_connections| acc + node_connections.len())
            / 2 // Connections are bi-directional
    }

    pub fn get_for(&self, id: usize) -> Option<&Vec<usize>> {
        self.connections.get(&id)
    }

    pub fn connect_nodes(&mut self, a: usize, b: usize) {
        self.add_connection(a, b);
        self.add_connection(b, a);
    }

    fn add_connection(&mut self, k: usize, v: usize) {
        match self.connections.get_mut(&k) {
            Some(k_connections) => {
                k_connections.push(v);
            }
            None => {
                let k_connections = vec![v];
                self.connections.insert(k, k_connections);
            }
        };
    }
}
