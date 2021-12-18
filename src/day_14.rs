use std::collections::HashMap;

use crate::input_parser::parse_input_at;

pub fn day_14_1() -> usize
{
    let (mut polymer, instructions) = read_input_from_file("src/inputs/day_14_input.txt");
    for _ in 0..10
    {
        expand(&mut polymer, &instructions);
    }
    let (most_common, less_common) = get_most_and_least_common_elements(polymer);

    return most_common.1 - less_common.1;
}

fn read_input_from_file(path: &str) -> (String, Vec<String>)
{
    let input = parse_input_at(path).unwrap_or_default();
    let mut entries = input.lines();

    let polymer = entries.nth(0)
                         .unwrap_or_default()
                         .to_string();

    entries.next(); // Skip empty line

    let mut insertions = Vec::new();
    for entry in entries
    {
        insertions.push(entry.to_string());
    }

    return (polymer, insertions);
}

fn expand(polymer: &mut String, instructions: &Vec<String>)
{
    let mut i = 0;
    while i < polymer.len() - 1
    {
        let pair = get_pair_at(polymer, i);

        for instruction in instructions
        {
            let (pattern, char_to_insert) = instruction.split_once(" -> ").unwrap_or_default();

            if pair == pattern
            {
                polymer.insert_str(i+1, char_to_insert);
                i += 1;
            }
        }
        i += 1;
    }
}

fn get_pair_at(polymer: &String, i: usize) -> String
{
    let j = i + 1;
    let first_char  = polymer.chars().nth(i).unwrap_or_default();
    let second_char = polymer.chars().nth(j).unwrap_or_default();

    let mut pair = String::new();
    pair.push(first_char);
    pair.push(second_char);

    return pair;
}

fn get_most_and_least_common_elements(polymer: String) -> ((char, usize), (char, usize))
{
    let mut counters: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars()
    {
        *counters.entry(c).or_insert(0) += 1;
    }

    let mut max = (' ', 0);
    let mut min = (' ', usize::MAX);

    for entry in counters
    {
        if entry.1 > max.1 { max = entry; }
        if entry.1 < min.1 { min = entry; }
    }

    return (max, min);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        let (mut polymer, instructions) = read_input_from_file("src/inputs/day_14_example.txt");
        for _ in 0..10
        {
            expand(&mut polymer, &instructions);
        }
        let (most_common, less_common) = get_most_and_least_common_elements(polymer);

        assert_eq!(most_common.1 - less_common.1, 1588);
    }

    #[test]
    fn test_read_input_polymer()
    {
        let (polymer, _) = read_input_from_file("src/inputs/day_14_example.txt");
        assert_eq!(polymer, "NNCB");
    }

    #[test]
    fn test_read_input_insertions()
    {
        let (_, insertions) = read_input_from_file("src/inputs/day_14_example.txt");
        assert_eq!(insertions.len(), 16);
    }

    #[test]
    fn test_expand()
    {
        let (mut polymer, instructions) = read_input_from_file("src/inputs/day_14_example.txt");
        expand(&mut polymer, &instructions);
        assert_eq!(polymer, "NCNBCHB");
    }
}
