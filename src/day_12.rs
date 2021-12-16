use std::{collections::{HashMap, HashSet}};

use crate::input_parser::parse_input_at;

pub fn day_12_1() -> usize
{
    let mut map = Map::new_from_file("src/inputs/day_12_input.txt");
    return map.get_num_of_possible_paths();
}

#[derive(Debug)]
struct Map
{
    nodes: HashMap<String, MapNode>
}

#[derive(Debug, Default)]
struct MapNode
{
    name: String,
    neighbours: Vec<String>,
    already_visited: bool
}

impl Map
{
    pub fn new_from_file(path: &str) -> Self
    {
        let input = parse_input_at(path).unwrap_or_default();

        let mut map = Self{ nodes: HashMap::new() };
        for line in input.lines()
        {
            let (node_name, neighbour_name) = line.split_once("-").unwrap_or_default();
            let node_name      = node_name.to_string();
            let neighbour_name = neighbour_name.to_string();

            let new_node  = map.nodes.entry(node_name.clone()).or_insert(MapNode::default());
            new_node.name = node_name.clone();
            new_node.neighbours.push(neighbour_name.clone());

            let new_neighbour  = map.nodes.entry(neighbour_name.clone()).or_insert(MapNode::default());
            new_neighbour.name = neighbour_name;
            new_neighbour.neighbours.push(node_name);
        }
        return map;
    }

    pub fn get_num_of_possible_paths(&mut self) -> usize
    {
        let mut path_count = 0;
        let mut visited_list = HashSet::new();
        let start_node = self.nodes.get("start").unwrap();
        for node_name in &start_node.neighbours
        {
            let node = self.nodes.get(node_name).unwrap();

            visited_list.clear();
            if is_small_cave(node_name)
            {
                visited_list.insert(node_name.clone());
            }

            path_count += self.count_paths_from(&node, &visited_list);
        }
        return path_count;
    }

    fn count_paths_from(&self, start_node: &MapNode, visited_so_far: &HashSet<String>) -> usize
    {
        let mut path_count = 0;
        for neighbour in &start_node.neighbours
        {
            // New path, new list, but keeping the ones that we had to visit to get to start_node
            let mut visited_list = visited_so_far.clone();

            if visited_list.contains(neighbour) { continue; }

            match neighbour.as_str()
            {
                "start" => continue,
                "end"   => { path_count += 1; continue; },
                _       => ()
            }

            if is_small_cave(&neighbour)
            {
                visited_list.insert(neighbour.clone());
            }

            if let Some(neighbour_node) = self.nodes.get(neighbour)
            {
                path_count += self.count_paths_from(&neighbour_node, &visited_list);
            }
        }

        return path_count;
    }
}

fn is_small_cave(name: &String) -> bool
{
    for c in name.chars()
    {
        if c.is_uppercase() { return false; }
    }

    return true;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle_1()
    {
        let mut map = Map::new_from_file("src/inputs/day_12_example_1.txt");
        assert_eq!(map.get_num_of_possible_paths(), 10);
    }

    #[test]
    fn test_first_puzzle_2()
    {
        let mut map = Map::new_from_file("src/inputs/day_12_example_2.txt");
        assert_eq!(map.get_num_of_possible_paths(), 19);
    }

    #[test]
    fn test_first_puzzle_3()
    {
        let mut map = Map::new_from_file("src/inputs/day_12_example_3.txt");
        assert_eq!(map.get_num_of_possible_paths(), 226);
    }

    #[test]
    fn test_create_map()
    {
        let map = Map::new_from_file("src/inputs/day_12_example_1.txt");
        assert_eq!(map.nodes.len(), 6);
    }
}