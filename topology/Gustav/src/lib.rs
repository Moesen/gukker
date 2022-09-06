use core::panic;
use std::{collections::HashMap, collections::HashSet, fmt, fs::File, io::{BufReader, BufRead}, path::{PathBuf, Path}, env};
use project_root;

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
        let all_v = self.get_vertices_keys();
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

pub fn get_testfile_path(filename: &str, input: bool) -> PathBuf {
    let root = project_root::get_project_root();
    let mut fp = match root {
        Ok(p) => p,
        Err(e) => panic!("{}", e)
    };
    fp.push("tests");
    match input {
        true=>fp.push("inputs"),
        false=>fp.push("outputs")
    }
    fp.push(filename);
    return fp
    
}

pub fn read_input(filename: &str) -> BufReader<File> {
    let fp = get_testfile_path(filename, true);
    let f = File::open(fp);
    match f {
        Ok(x)=>BufReader::new(x),
        Err(x)=>panic!("{}", x)
    }
}

pub fn read_output(filename: &str) -> (i32, i32) {
    let fp = get_testfile_path(filename, false);
    println!("{}", fp.to_str().unwrap());
    let f = File::open(fp).expect("File");
    let mut br = BufReader::new(f);
    let mut line = String::new();
    br.read_line(&mut line).expect("ReadLine");
    let nums: Result<Vec<i32>, _> = line.trim().split(' ').map(str::parse).collect();
    match nums {
        Ok(x)=>(x[0], x[1]),
        Err(e)=>panic!("{}", e)
    }
}
