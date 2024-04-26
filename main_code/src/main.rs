use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let l_fr = 6549;
    let n_fr: usize = 122666;
    let (adjacency_list_fr, list_of_edges_fr) = read_file("musae_FR_edges.txt", l_fr, n_fr);

    let fr_graph = Graph{vertices: l_fr, edges: adjacency_list_fr};

}


fn read_file(path: &str, l:usize, n: usize) -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let data = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(data).lines();
    let mut adjacency_list = vec![vec![];l];
    let mut list_of_edges: Vec<(i32, i32)> = vec![(0, 0); n];

    for (i,line) in reader.enumerate() {
        let line_str = line.expect("Error reading");
        let v:Vec<i32> = line_str.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        
        let v1 = v[0] as i32;
        let v1_usize = v[0] as usize;
        let v2 = v[1] as i32;
        adjacency_list[v1_usize].push(v2);
        list_of_edges[i] = (v1, v2);
        
    }
    return (adjacency_list, list_of_edges)
}



#[derive(Debug)]
struct Graph {
    vertices: usize,
    edges: Vec<Vec<i32>>
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
            self.edges[index].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.edges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n: usize, edges: &Vec<(i32, i32)>) -> Graph {
        let mut g = Graph{vertices: n, edges:vec![vec![];n]};
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

// Créer une fonction qui fait le chemin entre 2 noeuds

// Créer une fonction pour calculer la moyenne des chemins pour tous les noeuds

// Comparer les différentes moyennes entre les nationalités 