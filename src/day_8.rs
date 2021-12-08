use crate::input_parser::parse_input_at;

pub fn day_8_1(input_path: &str) -> usize
{
    let input = parse_input_at(input_path).unwrap_or_default();

    let mut count = 0;
    for entry in input.lines()
    {
        let out_blocks = get_output_blocks_from_entry(entry);
        count += count_numbers_with_unique_patterns(out_blocks);
    }
    return count;
}

fn get_output_blocks_from_entry(entry: &str) -> Vec<&str>
{
    let (_, output) = entry.split_once("|")
                           .unwrap_or_default();

    return output.split_whitespace()
                 .collect();
}

fn count_numbers_with_unique_patterns(blocks: Vec<&str>) -> usize
{
    let mut count = 0;
    for block in blocks
    {
        match block.chars().count()
        {
            2|3|4|7 => count += 1,
            _       => continue
        }
    }
    return count;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        assert_eq!(day_8_1("src/inputs/day_8_example.txt"), 26);
    }

    #[test]
    fn test_get_output_blocks_from_entry()
    {
        let entry = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(get_output_blocks_from_entry(entry).len(), 4);
    }

    #[test]
    fn test_count_numbers_with_unique_patterns()
    {
        let output_blocks = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];
        assert_eq!(count_numbers_with_unique_patterns(output_blocks), 2);
    }
}