// Link for the dataset : https://snap.stanford.edu/data/twitch-social-networks.html

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

pub mod six_degrees_module;
use six_degrees_module::six_degrees;


fn main() {
    //Dataset for French Twitch users
    let graph_fr = build_graph(6549, "musae_FR_edges.txt"); 
    print_results(&graph_fr, "French");

    //Dataset for English Twitch users
    let graph_en = build_graph(7126, "musae_ENGB_edges.txt");
    print_results(&graph_en, "English");

    //Dataset for Portugese Twitch users
    let graph_pt = build_graph(1912, "musae_PTBR_edges.txt");
    print_results(&graph_pt, "Portugese");

    let accuracy_fr = ((graph_fr.vertices as f64) - (six_degrees::computation_6_degrees(&graph_fr).1 as f64)) / (graph_fr.vertices as f64) * 100.0;
    let accuracy_en = ((graph_en.vertices as f64) - (six_degrees::computation_6_degrees(&graph_en).1 as f64)) / (graph_en.vertices as f64) * 100.0;
    let accuracy_pt = ((graph_pt.vertices as f64) - (six_degrees::computation_6_degrees(&graph_pt).1 as f64)) / (graph_pt.vertices as f64) * 100.0;

    let min_value = if accuracy_fr <= accuracy_en && accuracy_fr <= accuracy_pt {
        "French"
    } else if accuracy_en <= accuracy_fr && accuracy_en <= accuracy_pt {
        "English"
    } else {
        "Portuguese"
    };
    println!("The nationality respecting the least the rule of six degrees of seperation is {:?}", min_value);
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

fn print_results(graph: &six_degrees::Graph, nationality: &str) {
    let start_time = Instant::now(); 
    let average = six_degrees::computation_6_degrees(graph).0;
    let errors = six_degrees::computation_6_degrees(graph).1 as f64;

    print!("{}\n", nationality);
    println!("Average of degrees of seperation between two nodes: {:.5}", average);
    println!("The 6 degrees seperation rule is respected {:.2}% of the time", ((graph.vertices as f64) - errors) / (graph.vertices as f64) * 100.0);
    println!("Number of trips violating the rule: {}", errors as i32);

    let end_time = Instant::now();

    println!("Elapsed time: {:?}\n", end_time - start_time);
}

#[test]
fn check_distance_2_vertices() {
    let adjacency_list_test = vec![vec![1, 2], vec![0, 2, 4], vec![0, 1, 3], vec![2, 4], vec![1, 3]];
    let graph = six_degrees::Graph{vertices: 5, adjacency_list: adjacency_list_test};
    let test = six_degrees::distance_2_vertices(0, 4, &graph);
    assert_eq!(test, 2, "The computed distance is wrong")
}

#[test]
fn check_computation_6_degrees() {
    let adjacency_list_test = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6]];
    let graph = six_degrees::Graph{vertices: 8, adjacency_list: adjacency_list_test};
    let (accuracy, rule_violation) = six_degrees::computation_6_degrees(&graph);
    assert!(accuracy >= 0.0 && accuracy <= 6.0, "The accuracy is out of range");
    assert!(rule_violation >= 0 && rule_violation <= graph.vertices as i32, "The rule_violation number is out of range")
}

