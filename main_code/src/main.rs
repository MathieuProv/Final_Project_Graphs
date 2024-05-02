// Link for the dataset : https://snap.stanford.edu/data/twitch-social-networks.html

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

pub mod module;
use module::six_degrees;


fn main() {
    //Dataset for French Twitch users
    let graph_fr = build_graph(6549, "musae_FR_edges.txt"); 
    print_results(&graph_fr, "France");

    //Dataset for English Twitch users
    let graph_en = build_graph(7126, "musae_ENGB_edges.txt");
    print_results(&graph_en, "English");

    //Dataset for Portugese Twitch users
    let graph_pt = build_graph(1912, "musae_PTBR_edges.txt");
    print_results(&graph_pt, "Portugese")
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

fn build_graph(len: i32, text: &str) -> six_degrees::Graph {
    let mut list_of_edges = read_file(text);
    list_of_edges.sort();

    return six_degrees::Graph::create_undirected(len as usize, &list_of_edges);
}

fn print_results(graph: &six_degrees::Graph, country: &str) {
    let start_time = Instant::now(); 
    let average = six_degrees::computation_6_degrees(graph).0;
    let errors = six_degrees::computation_6_degrees(graph).1 as f64;

    print!("{}\n\n", country);
    println!("Average of degrees of seperation between two nodes: {:.5}", average);
    println!("The 6 degrees seperation rule is respected {:.2}% of the time", ((graph.vertices as f64) - errors) / (graph.vertices as f64) * 100.0);
    println!("Number of trips violating the rule: {}\n", errors as i32);

    let end_time = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}