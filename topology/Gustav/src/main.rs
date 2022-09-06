use std::{
    fs::File, 
    io::{self, BufReader},
};
use topology;

fn main() -> io::Result<()> {
    let fp = "tests/inputs/small_example.txt";
    let f = File::open(fp)?;
    let f = BufReader::new(f);
    let mut graph = topology::init_graph(f);
    println!("{}\n", graph);
    for (k, v) in graph.get_vertices() {
        println!("ID: {}, neighbors: {}", k, v);
    }
    let count = graph.count_continents();
    println!("{}", count);

    Ok(())
}
