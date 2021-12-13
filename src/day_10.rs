use crate::input_parser::parse_input_at;

pub fn day_10_1() -> usize
{
    let illegal_chars = get_illegal_chars_from_file("src/inputs/day_10_input.txt");
    return compute_error_score(&illegal_chars);
}

fn get_illegal_chars_from_file(path: &str) -> String
{
    let input = parse_input_at(path).unwrap_or_default();

    let mut illegals = String::default();
    for line in input.lines()
    {
        let c = get_first_illegal_char_in_line(line);
        if c.is_some()
        {
            illegals.push(c.unwrap());
        }
    }
    return illegals;
}

fn get_first_illegal_char_in_line(line: &str) -> Option<char>
{
    let mut stack = Vec::new();

    for c in line.chars()
    {
        if is_closing(&c)
        {
            if !matches_last_opening(&c, &stack.pop().unwrap_or('x'))
            {
                return Some(c);
            }
            continue;
        }
        stack.push(c);
    }
    return None;
}

fn is_closing(c: &char) -> bool
{
    match c
    {
        ')' | ']' | '}' | '>' => true,
        _ => false
    }
}

fn matches_last_opening(c: &char, o: &char) -> bool
{
    match c
    {
        ')' => match o { '(' => true, _ => false },
        ']' => match o { '[' => true, _ => false },
        '}' => match o { '{' => true, _ => false },
        '>' => match o { '<' => true, _ => false },
        _ => false
    }
}

fn compute_error_score(chars: &String) -> usize
{
    let mut score = 0;
    for c in chars.chars()
    {
        score += match c
        {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
             _  => 0
        };
    }
    return score;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        let illegal_chars = get_illegal_chars_from_file("src/inputs/day_10_example.txt");
        assert_eq!(compute_error_score(&illegal_chars), 26397);
    }

    #[test]
    fn test_get_illegal_chars()
    {
        assert_eq!(get_illegal_chars_from_file("src/inputs/day_10_example.txt"),
                   "})])>");
    }

    #[test]
    fn test_get_first_illegal_char_in_line()
    {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let first = get_first_illegal_char_in_line(line).unwrap();

        assert_eq!(first, '}');
    }

    #[test]
    fn test_compute_error_score()
    {
        let illegal_chars = "))]}>".to_string();
        assert_eq!(compute_error_score(&illegal_chars), 26397);
    }
}
