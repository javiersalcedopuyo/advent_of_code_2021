use crate::
{
    input_parser::*,
};

pub fn day_4_1() -> i32
{
    let input = parse_input_at("src/inputs/day_4_input.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    return get_first_winning_score(input);
}

pub fn day_4_2() -> i32
{
    let input = parse_input_at("src/inputs/day_4_input.txt");
    assert!(input.is_ok());
    let input = input.unwrap();

    return get_last_winning_score(input);
}

fn get_first_winning_score(input: String) -> i32
{
    let numbers = parse_drawn_nums(&input);
    let mut boards = parse_boards(&input);

    for num in numbers
    {
        for i in 0..boards.len()
        {
            if boards[i].check_and_mark(num)
            {
                return num * boards[i].get_unmarked_sum();
            }
        }
    }
    return 0;
}

fn get_last_winning_score(input: String) -> i32
{
    let numbers = parse_drawn_nums(&input);
    let mut boards = parse_boards(&input);

    let mut last_winning_number     = 0;
    let mut last_winning_board_idx  = 0;
    for num in numbers
    {
        for i in 0..boards.len()
        {
            if !boards[i].get_bingo_state() &&
                boards[i].check_and_mark(num)
            {
                last_winning_number     = num;
                last_winning_board_idx  = i;
            }
        }
    }
    return last_winning_number * boards[last_winning_board_idx].get_unmarked_sum();
}


#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_row_bingo()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let mut boards = parse_boards(&input);

        boards[0].check_and_mark(22);
        boards[0].check_and_mark(13);
        boards[0].check_and_mark(17);
        boards[0].check_and_mark(11);
        assert!(boards[0].check_and_mark(0));
    }

    #[test]
    fn test_col_bingo()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let mut boards = parse_boards(&input);

        boards[0].check_and_mark(22);
        boards[0].check_and_mark(8);
        boards[0].check_and_mark(21);
        boards[0].check_and_mark(6);
        assert!(boards[0].check_and_mark(1));
    }

    #[test]
    fn test_first_puzzle()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let score = get_first_winning_score(input);

        assert_eq!(score, 4512);
    }

    #[test]
    fn test_second_puzzle()
    {
        let input = parse_input_at("src/inputs/day_4_example.txt");
        assert!(input.is_ok());
        let input = input.unwrap();

        let score = get_last_winning_score(input);

        assert_eq!(score, 1924);
    }
}