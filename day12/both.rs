use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() -> () {
    let link_vecs = read_lines("input.txt");

    // Determine unique names in edge pairings
    let mut names = link_vecs.iter().cloned().flatten().collect::<Vec<_>>();
    names.sort();
    names.dedup();

    // Create empty graph using node names
    let mut graph = names.iter().map(
        |s| ((&s).to_string(), vec![])
    ).collect::<HashMap<_, Vec<String>>>();

    // Add edges to graph
    for pair in link_vecs {
        graph.get_mut(&pair[0]).unwrap().push(pair[1].clone());
        graph.get_mut(&pair[1]).unwrap().push(pair[0].clone());
    }

    // Begin search at start node, can only user lowercase nodes once
    println!("Part 1: {}", paths_to_end(&graph, String::from("start"), HashSet::new(), false));

    // Begin search at start node, with ability to ignore lowercase rule once
    println!("Part 2: {}", paths_to_end(&graph, String::from("start"), HashSet::new(), true));

}

fn is_lower(s : &String) -> bool {
    // Determines whether a string is lower case, that is, with all char values >= 97 (a)
    return s.chars().fold(true, |acc, c| acc && (c as u8 >= 97));
}

fn paths_to_end(graph: &HashMap<String, Vec<String>>, curr_node: String, mut history: HashSet<String>, twice: bool) -> i32 {
    // Recursive, traverses neighbours of current node if not in history
    // Note: guaranteed no loops, as finite number of routes wanted as answer

    // Base case, reached end node
    if curr_node.as_str() == "end" { return 1; }
    let mut paths = 0;

    // If lowercase, insert current node into history
    if is_lower(&curr_node) {
        history.insert(curr_node.clone());
    }

    // Explore all neighbouring paths
    for neighbour in &graph[&curr_node] {
        if !history.contains(neighbour) {
            paths += paths_to_end(graph, neighbour.to_string(), history.clone(), twice);
        } else if twice && neighbour.as_str() != "start" {
            // If 'twice' enabled, allows us to ignore adding to history once 
            paths += paths_to_end(graph, neighbour.to_string(), history.clone(), false);
        }
    }
    return paths;
}

fn read_lines<P>(filename: P) -> Vec<Vec<String>> where P: AsRef<Path> {
    // Converts input lines to vector of vector pairs
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    return lines
    .map(
        |l| l.unwrap()
        .split("-")
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
}