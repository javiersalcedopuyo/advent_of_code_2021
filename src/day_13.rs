use crate::input_parser::parse_input_at;

pub fn day_13_1() -> usize
{
    let (mut sheet, instructions) = get_paper_and_instructions_from_file("src/inputs/day_13_input.txt");
    let first_instruction = vec![instructions.first().unwrap().clone()];
    sheet.fold(first_instruction);
    return sheet.count_points();
}

pub fn day_13_2()
{
    let (mut sheet, instructions) = get_paper_and_instructions_from_file("src/inputs/day_13_input.txt");
    sheet.fold(instructions);
    sheet.print();
}

fn get_paper_and_instructions_from_file(path: &str) -> (PaperSheet, Vec<FoldInstruction>)
{
    let input = parse_input_at(path).unwrap_or_default();

    let mut sheet_points = Vec::new();
    let mut instructions = Vec::new();
    let mut parsing_sheet = true;
    for line in input.lines()
    {
        if line.is_empty() { parsing_sheet = false; continue; }

        if parsing_sheet
        {
            sheet_points.push( parse_point(line) );
        }
        else
        {
            instructions.push( parse_instruction(&line) );
        }
    }

    return (PaperSheet::new(sheet_points), instructions);
}

fn parse_point(entry: &str) -> Point
{
    let (x,y) = entry.split_once(",").unwrap_or_default();

    let x = x.parse::<usize>().unwrap_or_default();
    let y = y.parse::<usize>().unwrap_or_default();

    return Point{ x, y, };
}

fn parse_instruction(entry: &str) -> FoldInstruction
{
    let words = entry.split_whitespace();
    let relevant_data = words.last().unwrap_or_default();
    let (axis, pos) = relevant_data.split_once("=").unwrap_or_default();

    let axis = match axis
    {
        "x" => Axis::X,
        "y" => Axis::Y,
         _  => panic!("Unknown axis!")
    };

    let pos = pos.parse::<usize>().unwrap_or_default();

    return FoldInstruction{ axis, pos };
}

#[derive(Default)]
struct PaperSheet
{
    points: Vec<Vec<bool>>,
    width: usize,
    height: usize
}

#[derive(Default, Clone, Copy)]
struct Point
{
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct FoldInstruction
{
    axis: Axis,
    pos: usize
}

#[derive(Clone, Copy)]
enum Axis
{
    X = 0,
    Y
}

impl PaperSheet
{
    pub fn new(points: Vec<Point>) -> Self
    {
        let width  = points.iter().max_by(|a,b| { a.x.cmp(&b.x) } ).unwrap_or(&Point::default()).x + 1;
        let height = points.iter().max_by(|a,b| { a.y.cmp(&b.y) } ).unwrap_or(&Point::default()).y + 1;

        let mut sheet = Self{ points: Vec::with_capacity(width), width, height };
        sheet.points.resize(width, Vec::with_capacity(height));
        for col in &mut sheet.points
        {
            col.resize(height, false);
        }

        for p in points
        {
            sheet.points[p.x][p.y] = true;
        }
        return sheet;
    }

    pub fn get_size(&self) -> (usize, usize) { (self.width, self.height) }

    pub fn fold(&mut self, instructions: Vec<FoldInstruction>)
    {
        for instruction in instructions
        {
            // Resize
            match instruction.axis
            {
                Axis::X => self.width  = instruction.pos,
                Axis::Y => self.height = instruction.pos
            }

            // "Move" the points outside the fold
            for x in 0..self.points.len()
            {
                for y in 0..self.points[0].len()
                {
                    // println!(" X{} vs W{}", x, self.width);
                    let new_x = if x > self.width  && x <= 2*self.width  { 2*self.width  - x } else { x };
                    let new_y = if y > self.height && y <= 2*self.height { 2*self.height - y } else { y };

                    self.points[new_x][new_y] |= self.points[x][y];
                }
            }
        }
    }

    pub fn count_points(&self) -> usize
    {
        let mut count = 0;
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                if self.points[x][y] { count += 1; }
            }
        }
        return count;
    }

    pub fn print(&self)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                print!("{}", if self.points[x][y] { 'x' } else { ' ' });
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        let (mut sheet, instructions) = get_paper_and_instructions_from_file("src/inputs/day_13_example.txt");
        let first_instruction = vec![instructions.first().unwrap().clone()];
        sheet.fold(first_instruction);
        assert_eq!(sheet.count_points(), 17);
    }

    #[test]
    fn test_second_puzzle()
    {
        let (mut sheet, instructions) = get_paper_and_instructions_from_file("src/inputs/day_13_example.txt");
        sheet.fold(instructions);
        sheet.print();
        // panic!(); // Uncomment to see the print
    }

    #[test]
    fn test_get_sheet()
    {
        let (sheet, _) = get_paper_and_instructions_from_file("src/inputs/day_13_example.txt");
        assert_eq!(sheet.get_size(), (11, 15));
        assert_eq!(sheet.count_points(), 18);
    }

    #[test]
    fn test_get_instructions()
    {
        let (_, instructions) = get_paper_and_instructions_from_file("src/inputs/day_13_example.txt");
        assert_eq!(instructions.len(), 2);
    }
}