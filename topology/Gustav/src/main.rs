use std::io;
use topology;

fn main() -> io::Result<()> {
    let fp = "tests/inputs/small_example.txt";
    let input = topology::read_input(fp);
    let mut graph = topology::init_graph(input);
    println!("{}\n", graph);
    for (k, v) in graph.get_vertices() {
        println!("ID: {}, neighbors: {}", k, v);
    }
    // let count = graph.count_continents();
    // println!("{}", count);
    topology::read_output(&"tests/outputs/small_example.txt");

    Ok(())
}
