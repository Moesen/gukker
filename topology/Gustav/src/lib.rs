use std::{collections::HashMap, fmt, collections::HashSet};

#[derive(Debug, Clone)]
pub struct Vertex {
    id: i32,
    neighbors: Vec<i32>,
    visited: bool,
}

#[derive(Debug)]
pub struct Graph {
    vertices: HashMap<i32, Vertex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.vertices.entry(from).or_insert(Vertex {
            id: from,
            neighbors: Vec::new(),
            visited: false,
        });
        self.vertices.entry(to).or_insert(Vertex {
            id: to,
            neighbors: Vec::new(),
            visited: false,
        });
        if let Some(x) = self.vertices.get_mut(&from) {
            x.neighbors.push(to);
        };
        if let Some(x) = self.vertices.get_mut(&to) {
            x.neighbors.push(from);
        }
    }
    pub fn get_vertices(&self) -> HashMap<i32, Vertex> {
        self.vertices.clone()
    }
    pub fn get_vertices_keys(&self) -> HashSet<i32> {
        self.vertices.keys().cloned().collect()
    }

    pub fn count_continents(&mut self) -> i32 {
        let mut all_v = self.get_vertices_keys();
        let mut q: Vec<i32> = Vec::new();
        let mut count = 0;
        while all_v.len() > 0 {
            if q.len() == 0 {
                count += 1;
            }
            let v_idx: i32 = q.pop().expect("Somehow a none value was found");
            let mut vertex = self.vertices.get_mut(&v_idx).unwrap();
            if vertex.visited == true {
                panic!("Something")
            }
            vertex.visited = true;
            
        }
        count
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Num vertices: {}", self.vertices.len())
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}, neighbors: {:?}", self.id, self.neighbors)
    }
}
pub fn init_graph(f: impl std::io::BufRead) -> Graph {
    let mut graph = Graph::new();
    for line in f.lines() {
        let input = line.unwrap();
        let inputs: Vec<&str> = input.split(" ").collect();
        if inputs.len() <= 1 {
            continue;
        }
        let from: i32 = inputs[0].parse::<i32>().unwrap();
        let to: i32 = inputs[1].parse::<i32>().unwrap();
        graph.add_edge(from, to);
    }
    return graph;
}
