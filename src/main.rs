use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
type Weight = f32;
use std::cmp::Reverse;
use rand::Rng;


mod importing;


#[derive(Debug, Clone)]
struct Node {
    id: usize,
    weight: Weight,
    edges: Vec<(usize, Weight)>,
}

impl Node {
    fn new(id: usize, weight: Weight) -> Self {
        Node {
            id,
            weight,
            edges: vec![],
        }
    }

    fn new_edge(&mut self, to: usize, weight: Weight) {
        self.edges.push((to, weight));
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Graph {
            nodes: vec![Node::new(0, 0.0); num_nodes],
        }
    }

    fn add_node(&mut self, id:usize, weight: Weight) {
        self.nodes.push(Node::new(id, weight));
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: Weight) {
        self.nodes[from].new_edge(to, weight);
        self.nodes[to].new_edge(from, weight);
    }

}


    fn build_nodes(graph: Graph, file_path: &str) -> Graph{ 
        
        let mut map: Graph = graph;

        //file reading section
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let values: Vec<&str> = line.split(',').collect();
            let _name = values[0].to_string();
            let w: f32 = values[3].parse().unwrap();
            let i: f32 = values[4].parse().unwrap();
            map.add_node(i as usize, w);
        }
        
        map
    }


    fn build_edges(file_path: &str, coord_file_path: &str, graph: Graph) -> Graph {

        let coords: HashMap<String, Vec<f32>> = importing::read_coords(coord_file_path);

        let mut map: Graph = graph;

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            //get your line
            let v: Vec<&str> = line.split(',').collect(); 
            let stop_a = v[0]; // name of stop a
            let stop_b = v[1]; //name of stop b 
            //get the real coords of both stops
            let coord_a = &coords[stop_a]; //coords for stop a
            let coord_b = &coords[stop_b]; //coords for stop b
            //calculate the distance between them (this is the formula for latitude and longitude points on a global scale)
            let dist = ((coord_a[0]).sin() * (coord_b[0]).sin() + (coord_a[0]).cos() * (coord_b[0]).cos() * (coord_b[1] - coord_a[1]).cos()).acos()*3958.8;
            //get index of each place
            let index_a = coords[stop_a][2] as usize;
            let index_b = coords[stop_b][2] as usize;
            //add the edge
            map.add_edge(index_a, index_b, dist);
        }
        map

    }






    fn dijkstra(map: &Graph, source: usize, target: usize) -> Option<Vec<usize>> {
        let graph = &map.nodes;
        let mut heap = BinaryHeap::new();
        let mut distances = vec![i32::MAX; graph.len()];
        let mut prev = vec![None; graph.len()];

        distances[source] = 0;
        heap.push(Reverse((0, source)));

        while let Some(Reverse((dist, n))) = heap.pop() {

            let cap:i64 = 2147483647*1;
            let mut sum_distances: i64 = 0;
            for d in &distances{
                sum_distances = sum_distances + *d as i64;
            }

            if sum_distances < cap { //the program was originally designed to stop when it hit the randomly generated end point, but I wanted to adjust it to keept going
                break; //the program will stop trying to find new nodes for the path when the sum of all values in the distances table hits the above given number
            } // that basically means that the path has run out of options to go to (without repeating)

            if dist > distances[n] {
                continue;
            }

            for edge in &graph[n].edges {
                let next = edge.0;
                let edge_weight = edge.1;
                let node_weight = &graph[n].weight;

                let weight = (edge_weight * (1.0/node_weight)) as i32;
                let next_dist = dist + weight;

                if next_dist < distances[next] {
                    distances[next] = next_dist;
                    prev[next] = Some(n);
                    heap.push(Reverse((next_dist, next)));
                }
            }
        }

        if prev[target].is_none() {
            return None;
        }

        let mut path = vec![target];
        let mut n = target;

        while let Some(prev_node) = prev[n] {
            path.push(prev_node); 
            n = prev_node; //gets added in reverse order
        }
        path.reverse(); 

        Some(path) //return path
    }




    fn run_dijkstra(graph:Graph) -> Vec<(Vec<usize>, f32)> {

        let mut paths:Vec<(Vec<usize>, f32)> = [].to_vec();
        for _x in 0..100 { //runs this section 100 times to get 100 paths (running it several times and then getting the one with the largest score gets you closer to the best possible)
                let mut rng = rand::thread_rng();
                let source = rng.gen_range(0..graph.nodes.len()); //generates a random start and random end
                let target = rng.gen_range(0..graph.nodes.len()); //the locaiton of the 'end' doesn't really mamtter becuase its usually not the actual end

                match dijkstra(&graph, source, target) { 
                    Some(path) => {
                            let mut score:f32 = 0.0;
                            for n in &path{
                                score  = score + &graph.nodes[*n].weight; //adds the path to the list along side the score in a tuple
                            }
                            paths.push((path, score));
                            //return path
                        }
                    None => { //there are some paths which start at a point which doesn't conect them to any other node, the program will return none
                    } //this will just return nothing and skip to the next path
                }
        }
        return paths

    }


    fn length_test(){ // due to the nature of the data that I was working with, I could not have a dataset which had 1000+ nodes
        //To prove that this program could potentially function in this same way with a larger data set, I created an extended testing set of coordinates
        //The rest of this function follows everything that the main function does but with that new data
        //the output of this program as no significance
        let mut test = Graph::new(0);

        test = build_nodes(test, "coordinates_testset.csv");
        test = build_edges("edges.csv", "coordinates_testset.csv", test);
        let paths = run_dijkstra(test); // gives 100 random paths
        let mut end_product:(Vec<usize>, f32) = ([].to_vec(), 0.0);
        for (p, s) in paths{
            if s > end_product.1{
                end_product = (p, s);
            }
        }
        println!("Test for larger data set (produced path has no meaning) : {:?}", end_product);
    }





fn main() {
    let mut mbta = Graph::new(0); //initializing the mbta map


    mbta = build_nodes(mbta, "coords.csv"); //adding the nodes from the coordinate points (adding weights/ridership with this)
    mbta = build_edges("edges.csv", "coords.csv", mbta); //adding edges


    let paths = run_dijkstra(mbta); // gives a list of 100 "best paths"

    let mut end_product:(Vec<usize>, f32) = ([].to_vec(), 0.0);

    for (p, s) in paths{
        if s > end_product.1{
            end_product = (p, s);
        }
    } //finds the path from the above list with the highest score

    let names = importing::read_names("coords.csv");


    println!("Produced New Path : {:?}", end_product);
    
    for n in end_product.0{
        println!("{}", names[n]);
    }

    length_test();


}
