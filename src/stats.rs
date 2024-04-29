use std::collections::HashMap;

// Calculates the minimum value and its corresponding key in a hash map of vectors.
// Returns None if the map is empty.
pub fn calculate_min(map: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
    // Return None immediately if the map is empty.
    if map.is_empty() {
        return None;
    }

    // Initialize minimum key and value with the first element's data.
    let mut min_key = *map.keys().next().unwrap();
    let mut min_value = *map.get(&min_key).unwrap().iter().next().unwrap();

    // Iterate through all key-value pairs in the map.
    for (&key, values) in map.iter() {
        // Find the minimum value in the current vector.
        if let Some(&value) = values.iter().min() {
            // Update the minimum key and value if a new minimum is found.
            if value < min_value {
                min_key = key;
                min_value = value;
            }
        }
    }

    // Return the minimum key and its corresponding value.
    Some((min_key, min_value))
}

// Calculates the maximum value and its corresponding key in a hash map of vectors.
// Returns None if the map is empty.
pub fn calculate_max(map: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
    // Return None if the map is empty.
    if map.is_empty() {
        return None;
    }

    // Initialize maximum key and value with the first element's data.
    let mut max_key = *map.keys().next().unwrap();
    let mut max_value = *map.get(&max_key).unwrap().iter().next().unwrap();

    // Iterate through all key-value pairs in the map.
    for (&key, values) in map.iter() {
        // Find the maximum value in the current vector.
        if let Some(&value) = values.iter().max() {
            // Update the maximum key and value if a new maximum is found.
            if value > max_value {
                max_key = key;
                max_value = value;
            }
        }
    }

    // Return the maximum key and its corresponding value.
    Some((max_key, max_value))
}

// Calculates the mean of all values in a hash map of vectors.
// Returns a tuple containing the total count of values and the mean.
pub fn calculate_mean(map: &HashMap<usize, Vec<usize>>) -> (usize, f64) {
    // Return (0, 0.0) if the map is empty.
    if map.is_empty() {
        return (0, 0.0);
    }

    // Initialize sum and count variables.
    let mut sum = 0;
    let mut count = 0;

    // Sum all values in the map and count them.
    for values in map.values() {
        for &value in values {
            sum += value;
            count += 1;
        }
    }

    // Calculate the mean as a floating point number.
    let mean = sum as f64 / count as f64;

    // Return the count of values and the calculated mean.
    (count, mean)
}

