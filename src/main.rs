use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;


mod bfs;
use crate::bfs::bfs;
mod stats;
use crate::stats::{calculate_min, calculate_max, calculate_mean};

fn main() -> io::Result<()> {
    // Read edges from a file to create a random subset of edges.
    let edges: Vec<(usize, usize)> = read_file("C:\\Users\\Daniel\\Documents\\210 Rust HW\\Project\\project\\src\\roadNet-CA.txt", 10000)?;
    // Convert the list of edges into an adjacency list for graph representation.
    let adjacency_list = adj_list(&edges);
    println!("Adjacency List: {:?}", adjacency_list);

    // Perform BFS for each node in the adjacency list to find shortest paths.
    for (&start_node, _) in adjacency_list.iter() {
        let distances = bfs(&adjacency_list, start_node);
        for (&end_node, &distance) in distances.iter() {
            match distance {
                Some(steps) => println!("Shortest path from {:?} to {:?} takes {:?} steps.", start_node, end_node, steps),
                None => println!("No path from {:?} to {:?} exists.", start_node, end_node),
            }
        }
    }

    // Calculate and display statistical data about the adjacency list.
    println!("Min: {:?}", calculate_min(&adjacency_list));
    println!("Max: {:?}", calculate_max(&adjacency_list));
    println!("Mean: {:?}", calculate_mean(&adjacency_list));

    Ok(())
}


// Function to read a specified number of lines from a file randomly.
fn read_file(path: &str, num_lines: usize) -> io::Result<Vec<(usize, usize)>> {
    // Open the file for reading.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Initialize random number generator.
    let mut rng = thread_rng();
    let mut reservoir: Vec<(usize, usize)> = Vec::with_capacity(num_lines);
    // Reservoir sampling to pick random lines.
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if index < num_lines {
                    reservoir.push((x, y));
                } else {
                    let i = rng.gen_range(0..=index);
                    if i < num_lines {
                        reservoir[i] = (x, y);
                    }
                }
            }
        }
    }

    Ok(reservoir)
}

// Function to convert edge pairs into an adjacency list.
fn adj_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    // Populate the adjacency list.
    for &(source, target) in edges {
        adjacency_list.entry(source).or_insert_with(Vec::new).push(target);
    }

    adjacency_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        // Create a small graph where the distances will clearly be 0, 1, and 2.
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]); // Node 1 is connected to Node 2
        graph.insert(2, vec![3]); // Node 2 is connected to Node 3
        graph.insert(3, vec![]);  // Node 3 has no outgoing connections

        let results = bfs(&graph, 1);
        
        // Check if the distance from Node 1 to itself is 0
        assert_eq!(results.get(&1), Some(&Some(0)));
        // Check if the distance from Node 1 to Node 2 is 1
        assert_eq!(results.get(&2), Some(&Some(1)));
        // Check if the distance from Node 1 to Node 3 is 2
        assert_eq!(results.get(&3), Some(&Some(2)));
    }
}
