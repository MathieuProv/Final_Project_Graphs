pub mod six_degrees {
    use std::collections::VecDeque;
    use rand::Rng;

    #[derive(Debug)]
    pub struct Graph {
        pub vertices: usize,
        pub adjacency_list: Vec<Vec<i32>>
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
    
        pub fn create_undirected(n: usize, edges: &Vec<(i32, i32)>) -> Graph {
            let mut g = Self::create_directed(n,edges);
            g.add_directed_edges(&reverse_edges(edges.to_vec()));
            g.sort_graph_lists();
            g                                        
        }
    }

    pub fn distance_2_vertices(start: i32, terminal: usize, graph: &Graph) -> i32 {
        let mut distance: Vec<i32> = vec![-1;graph.vertices];
        let mut queue = VecDeque::new();

        queue.push_back(start as usize);
        distance[start as usize] = 0;
    
        while let Some(v) = queue.pop_front() {
            if v == terminal as usize {
                return distance[terminal as usize] as i32;
            }
            for u in graph.adjacency_list[v].iter() {
                if distance[*u as usize] == -1 {
                    distance[*u as usize] = distance[v]+ 1;
                    queue.push_back(*u as usize);
                }
            }
        }
        return distance[terminal as usize] as i32
    }

    pub fn computation_6_degrees(graph: &Graph) -> (f64, i32) {
        let len: i32 = graph.vertices as i32;
        let mut sum: f64 = 0.0;
        let mut rule_violation = 0;

        for v in 0..len {
            let vector = v as i32;
            for _ in 0..10 {
                let u = rand::thread_rng().gen_range(0..graph.vertices);
                let distance = distance_2_vertices(vector, u, graph);                
                sum += distance as f64;
                if distance > 6 {
                    rule_violation += 1;
                }
            }
        }
        return (sum / (len as f64 * 10.0), rule_violation)
    }
}