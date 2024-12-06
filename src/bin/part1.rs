use std::collections::HashMap;
use input::read_stdin;

#[path = "../input.rs"]
mod input;

fn main() {
    let (l1, l2) = read_lists(&read_stdin());
    println!("total distance: {}", list_distance(&l1, &l2));
    println!("similarity score: {}", list_similarity(&l1, &l2));
}

fn read_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    // Collect values
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (v1, v2) = line.split_once("   ").unwrap();
        l1.push(v1.parse().unwrap());
        l2.push(v2.parse().unwrap());
    }

    // Smallest values come first!
    l1.sort();
    l2.sort();

    (l1, l2)
}

fn list_distance(l1: &[u32], l2: &[u32]) -> u32 {
    // Make sure lists were read in correctly
    let l1_len = l1.len();
    let l2_len = l2.len();
    assert!(l1_len == l2_len);

    // Get distance
    let mut dist: u32 = 0;
    for i in 0..l1_len {
        dist += l1[i].abs_diff(l2[i]);
    }
    dist
}

fn list_similarity(l1: &[u32], l2: &[u32]) -> u32 {
    // Build a map of how often each number occurs
    let frequencies: HashMap<u32, u32> = l2.iter().copied().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_default() += 1;
        map
    });

    // Calculate and return similarity
    l1.iter().fold(0, |similarity, value| {
        similarity + frequencies.get(value).unwrap_or(&0u32) * value
    })
}
