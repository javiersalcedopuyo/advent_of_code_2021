use crate::input_parser::parse_input_at;

use std::
{
    collections::HashMap,
    cmp::{max, min}
};

pub fn day_5_1() -> usize
{
    let input = parse_input_at("src/inputs/day_5_input.txt")
                    .unwrap_or("".to_string());
    let cells = map_line_segments(input);

    return count_overlapped_cells(cells);
}

fn count_overlapped_cells(cells: HashMap<(usize, usize), usize>) -> usize
{
    return cells.values()
                .filter(|&value| { *value > 1 } )
                .count();
}

fn map_line_segments(input: String) -> HashMap<(usize, usize), usize>
{
    let mut segments = HashMap::new();

    for entry in input.lines()
    {
        let segment = parse_input_entry(entry);

        if segment.is_horizontal()
        {
            for i in segment.start.0 .. segment.end.0+1
            {
                *segments.entry((i, segment.start.1)).or_insert(0) += 1;
            }
        }
        else if segment.is_vertical()
        {
            for i in segment.start.1 .. segment.end.1+1
            {
                *segments.entry((segment.start.0, i)).or_insert(0) += 1;
            }
        }
    }

    return segments;
}

fn parse_input_entry(entry: &str) -> LineSegment
{
    let (start, end) = entry.split_once(" -> ").unwrap_or( ("","") );
    let (x0, y0)     = start.split_once(',').unwrap_or( ("0", "0") );
    let (x1, y1)     = end.split_once(',').unwrap_or( ("0", "0") );

    let x0 = x0.parse::<usize>().unwrap_or(0);
    let x1 = x1.parse::<usize>().unwrap_or(0);
    let y0 = y0.parse::<usize>().unwrap_or(0);
    let y1 = y1.parse::<usize>().unwrap_or(0);

    return LineSegment
    {
        start:
        (
            min(x0, x1),
            min(y0, y1)
        ),
        end:
        (
            max(x0, x1),
            max(y0, y1)
        )
    };
}

struct LineSegment
{
    start: (usize, usize),
    end: (usize, usize)
}

impl LineSegment
{
    pub fn is_horizontal(&self) -> bool { self.start.1 == self.end.1 }
    pub fn is_vertical(&self)   -> bool { self.start.0 == self.end.0 }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_input_entry()
    {
        let entry = "0,0 -> 1,1";

        let segment = parse_input_entry(entry);

        assert_eq!(segment.start, (0,0));
        assert_eq!(segment.end,   (1,1));
    }

    #[test]
    fn test_parse_right_to_left_line()
    {
        let entry = "1,1 -> 0,0";

        let segment = parse_input_entry(entry);

        assert_eq!(segment.start, (0,0));
        assert_eq!(segment.end,   (1,1));
    }

    #[test]
    fn test_map_line_segments()
    {
        let input = parse_input_at("src/inputs/day_5_example.txt").unwrap();
        let cells = map_line_segments(input);

        assert_eq!(cells.len(), 21);
    }

    #[test]
    fn test_first_example()
    {
        let input = parse_input_at("src/inputs/day_5_example.txt").unwrap();
        let cells = map_line_segments(input);
        let count = count_overlapped_cells(cells);

        assert_eq!(count, 5);
    }
}