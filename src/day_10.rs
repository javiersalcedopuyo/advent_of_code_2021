use crate::input_parser::parse_input_at;

pub fn day_10_1() -> usize
{
    let illegal_chars = get_illegal_chars_from_file("src/inputs/day_10_input.txt");
    return compute_error_score(&illegal_chars);
}

pub fn day_10_2() -> usize
{
    let incomplete_lines = get_missing_chars_in_lines_from_file("src/inputs/day_10_input.txt");
    let scores = compute_autocomplete_scores(&incomplete_lines);
    return get_autocomplete_winner(scores);
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

fn get_missing_chars_in_lines_from_file(path: &str) -> Vec<String>
{
    let input = parse_input_at(path).unwrap_or_default();

    let mut lines = Vec::new();
    for line in input.lines()
    {
        let missing_chars = get_missing_chars_in_line(&line);
        if !missing_chars.is_empty()
        {
            lines.push(missing_chars);
        }
    }

    return lines;
}

fn get_missing_chars_in_line(line: &str) -> String
{
    let mut stack = Vec::new();
    for c in line.chars()
    {
        if is_closing(&c)
        {
            if !matches_last_opening(&c, &stack.pop().unwrap_or('x'))
            {
                // Corrupt line, skip
                return String::default();
            }
            continue;
        }
        stack.push(c);
    }

    let mut missing = String::default();
    for i in (0..stack.len()).rev()
    {
        let c = match stack[i]
        {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
             char  => panic!("Unexpected char: {}", char)
        };
        missing.push(c);
    }
    return missing;
}

fn compute_autocomplete_scores(missing_chars_per_line: &Vec<String>) -> Vec<usize>
{
    let mut scores = Vec::with_capacity(missing_chars_per_line.len());
    for line in missing_chars_per_line
    {
        scores.push( compute_autocomplete_score(&line) );
    }
    return scores;
}

fn compute_autocomplete_score(missing: &String) -> usize
{
    let mut score = 0;
    for char in missing.chars()
    {
        let points = match char
        {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
             _  => 0
        };
        score = score * 5 + points;
    }
    return score;
}

fn get_autocomplete_winner(mut scores: Vec<usize>) -> usize
{
    assert!(scores.len() % 2 != 0); // As said in the spec
    scores.sort_unstable();

    return scores[ scores.len() / 2 ];
}

#[cfg(test)]
mod tests
{
    use super::*;

    // FIRST PUZZLE
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

    // SECOND PUZZLE
    #[test]
    fn test_second_puzzle()
    {
        let incomplete_lines = get_missing_chars_in_lines_from_file("src/inputs/day_10_example.txt");
        let scores = compute_autocomplete_scores(&incomplete_lines);
        assert_eq!(get_autocomplete_winner(scores), 288957);
    }

    #[test]
    fn test_get_missing_chars_in_lines_from_file()
    {
        let incomplete_lines = get_missing_chars_in_lines_from_file("src/inputs/day_10_example.txt");
        assert_eq!(incomplete_lines.len(), 5);
    }

    #[test]
    fn test_get_missing_chars_in_line()
    {
        assert_eq!(get_missing_chars_in_line("[({(<(())[]>[[{[]{<()<>>"),
                   "}}]])})]");
    }

    #[test]
    fn test_get_missing_chars_in_corrupted_line()
    {
        assert!(get_missing_chars_in_line("{([(<{}[<>[]}>{[]{[(<()>").is_empty());
    }

    #[test]
    fn test_compute_autocomplete_score_of_line()
    {
        let missing = "}}]])})]".to_string();
        assert_eq!(compute_autocomplete_score(&missing), 288957);
    }

    #[test]
    fn test_get_autocomplete_winner()
    {
        let scores = vec![288957, 5566, 1480781, 995444, 294];
        assert_eq!(get_autocomplete_winner(scores), 288957);
    }
}
