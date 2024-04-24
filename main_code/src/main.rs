use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let l_fr = 6549;

    let adjacency_list_fr = read_file("musae_FR_edges.txt", l_fr);



}

fn read_file(path: &str, l:usize) -> Vec<Vec<i32>> {
    let data = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(data).lines();
    let mut adjacency_list = vec![vec![];l];

    for (_,line) in reader.enumerate() {
        let line_str = line.expect("Error reading");
        let v:Vec<i32> = line_str.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        
        let v1 = v[0] as usize;
        let v2 = v[1] as i32;
        adjacency_list[v1].push(v2)
        
    }
    return adjacency_list
}
