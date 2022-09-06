use std::io;

use std::collections::HashMap;
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // MK
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let e = parse_input!(input_line, i32);
    
    // Graph traversal
    let mut graph: HashMap<i32, Vec<_>> = HashMap::with_capacity(e as usize);
    
    for _i in 0..e as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let n_1 = parse_input!(inputs[0], i32);
        let n_2 = parse_input!(inputs[1], i32);
        
        graph.entry(n_1).and_modify(|x| {x.push(n_2)}).or_insert(vec![n_2]);
        graph.entry(n_2).and_modify(|x| {x.push(n_1)}).or_insert(vec![n_1]);
    }

    // Initialization of variables
    let mut visited = vec![false; graph.keys().len()];
    let mut queue: Vec<i32> = Vec::with_capacity(graph.keys().len());
    let mut connected_graphs: Vec<(i32, i32)> = Vec::new();
    let mut data: (i32, i32) = (0,0);

    // Continents
    let mut c: usize = 0;

    // BFS+
    while visited.contains(&false) {
        c += 1; 
        queue.push(visited.iter().position(|&x| x == false).unwrap() as i32);
        while queue.len() != 0 {
            let vertex = queue.pop().unwrap() as usize;
            data.0 += 1;

            let _ = std::mem::replace(&mut visited[vertex], true);

            let lst = graph.get(&(vertex as i32)).unwrap();

            for i in lst {
                if visited[*i as usize] == false  && !queue.contains(&i){
                    queue.push(*i);
                }
            }
            data.1 += lst.len() as i32;
        }
        data.1 /= 2;
        connected_graphs.push(data);
        data = (0,0);
    }

    // Tiles
    let mut t: i32 = 0;

    for (vertices, edges) in &connected_graphs {
        t += 1 - vertices + edges;
    }

    println!("{} {}", c, t);
}
