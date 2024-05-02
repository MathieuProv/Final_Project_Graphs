use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;
use std::time::Instant;
use rand::{thread_rng, Rng};

fn main() {
    //Dataset for frenh Twitch users
    let l_fr = 6549;
    let mut list_of_edges_fr = read_file("musae_FR_edges.txt");
    list_of_edges_fr.sort();

    let fr_graph = Graph::create_undirected(l_fr, &list_of_edges_fr);

    print_results(&fr_graph, "France");
}


fn read_file(path: &str) -> Vec<(i32, i32)> {
    let data = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(data).lines();
    let mut list_of_edges: Vec<(i32, i32)> = vec![];

    for (_,line) in reader.enumerate() {
        let line_str = line.expect("Error reading");
        let v:Vec<i32> = line_str.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        list_of_edges.push((v[0], v[1]));
    }
    return list_of_edges
}

fn print_results(graph: &Graph, country: &str) {
    let start_time = Instant::now(); 

    print!("{}\n", country);
    println!("Average of degrees of seperation between two nodes: {:.5}\nNumber of trips violating the rule: {}", computation_6_degrees(graph).0, computation_6_degrees(graph).1);

    let end_time = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}


#[derive(Debug)]
struct Graph {
    vertices: usize,
    adjacency_list: Vec<Vec<i32>>
}


fn reverse_edges(list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((v,u));
    }
    new_list
}


impl Graph {
    fn add_directed_edges(&mut self, edges: &Vec<(i32, i32)>) {
        for (u,v) in edges {
            let index = *u as usize;
            self.adjacency_list[index].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.adjacency_list.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n: usize, edges: &Vec<(i32, i32)>) -> Graph {
        let mut g = Graph{vertices: n, adjacency_list:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n: usize, edges: &Vec<(i32, i32)>) -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges.to_vec()));
        g.sort_graph_lists();
        g                                        
    }
}


fn distance_2_vertices(start: i32, terminal: i32, graph: &Graph) -> i32 { // ISSUE : we're computing too much
    let mut distance: Vec<Option<i32>> = vec![None;graph.vertices];
    let mut queue = VecDeque::new();

    queue.push_back(start as usize);
    distance[start as usize] = Some(0);
    
    while let Some(v) = queue.pop_front() {
        if v == terminal as usize {
            return distance[terminal as usize].unwrap() as i32;
        }
        for u in graph.adjacency_list[v].iter() {
            if let None = distance[*u as usize] {
                distance[*u as usize] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u as usize);
            }
        }
    }
    return distance[terminal as usize].unwrap() as i32
}

fn computation_6_degrees(graph: &Graph) -> (f64, i32) {
    let len: i32 = graph.vertices as i32;
    let mut res: f64 = 0.0;
    let mut rule_violation = 0;
    for v in 0..len {
        let vector = v as i32;
        let terminal_vector: i32 = thread_rng().gen_range(0..len);
        res += distance_2_vertices(vector, terminal_vector, graph) as f64;
        if distance_2_vertices(vector, terminal_vector, graph) > 6 {
            rule_violation += 1;
        }
    }
    return (res / ((len as f64)), rule_violation)
}
