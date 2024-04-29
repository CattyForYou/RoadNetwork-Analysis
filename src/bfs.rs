use std::collections::{HashMap, VecDeque};

pub fn bfs(adjacency_list: &HashMap<usize, Vec<usize>>, start_node: usize) -> HashMap<usize, Option<usize>> {
    let mut shortest_paths = HashMap::new(); // Stores the shortest path distances.
    let mut visited = HashMap::new();        // Tracks visited nodes.
    let mut queue = VecDeque::new();         // Queue to manage the BFS frontier.

    // Initialize the BFS: mark the start node as visited and enqueue it with distance 0.
    visited.insert(start_node, true);
    queue.push_back((start_node, 0));
    shortest_paths.insert(start_node, Some(0)); // Distance to self is zero.

    while let Some((current_node, current_distance)) = queue.pop_front() {
        // Iterate through each neighbor of the current node.
        if let Some(neighbors) = adjacency_list.get(&current_node) {
            for &neighbor in neighbors {
                // If the neighbor has not been visited, mark it as visited.
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, true);
                    // Enqueue the neighbor with the incremented distance.
                    queue.push_back((neighbor, current_distance + 1));
                    // Record the distance to this neighbor.
                    shortest_paths.insert(neighbor, Some(current_distance + 1));
                }
            }
        }
    }

    shortest_paths
}
