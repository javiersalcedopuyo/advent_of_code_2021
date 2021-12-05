use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use crate::bingo_board::*;

pub fn parse_input_at(path_str: &str) -> io::Result<String>
{
    let path = Path::new(path_str);
    let mut file = File::open(path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    return Ok(buf);
}

pub fn parse_drawn_nums(input: &String) -> Vec<i32>
{
    let first_line = input.lines()
                          .nth(0)
                          .unwrap_or("")
                          .to_string();

    let mut result: Vec<i32> = Vec::new();
    for elem in first_line.split(",")
    {
        result.push( elem.parse::<i32>().unwrap_or(0) );
    }
    return result;
}

pub fn parse_boards(input: &String) -> Vec<BingoBoard>
{
    let mut result: Vec<BingoBoard> = Vec::new();

    let mut input_lines = input.lines();

    input_lines.next();
    while input_lines.next().is_some()
    {
        let mut new_board: BingoBoard = BingoBoard::default();
        for y in 0..5
        {
            let line = input_lines.nth(0)
                                  .unwrap_or("")
                                  .to_string();
            let mut x = 0;
            for elem in line.trim().split_whitespace()
            {
                let cell_value = elem.parse::<i32>()
                                     .unwrap_or(0);

                new_board.set_cell_at(x, y, cell_value);
                x += 1;
            }
        }

        result.push(new_board);
    }

    return result;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_load_example_file()
    {
        let path = Path::new("src/inputs/day_2_example.txt");
        let _ = match File::open(path)
        {
            Err(why) => panic!("Couldn't open because: {}", why),
            Ok(file) => file,
        };
    }

    #[test]
    fn test_parse_example_file()
    {
        let contents = parse_input_at("src/inputs/day_2_example.txt");
        assert!(contents.is_ok());

        let contents = contents.unwrap();

        let lines = contents.split("\n");
        let mut count = 0;
        for _ in lines
        {
            count += 1;
        }

        assert_eq!(count, 6);
    }

    #[test]
    fn test_parse_drawn_nums()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let nums = parse_drawn_nums(&input);
        assert_eq!(nums.len(), 27);
    }

    #[test]
    fn test_parse_boards()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let boards = parse_boards(&input);
        assert_eq!(boards.len(), 3);
    }
}