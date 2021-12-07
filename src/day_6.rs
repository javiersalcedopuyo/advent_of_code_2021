use crate::input_parser::parse_input_at;

pub fn day_6_1() -> usize
{
    let fishes = create_fishes_from_file("src/inputs/day_6_input.txt");
    return calculate_fish_population_after_n_days(fishes, 80);
}

fn create_fishes_from_file(path: &str) -> Vec<Fish>
{
    let input = parse_input_at(path)
                    .unwrap_or("".to_string());

    let mut fishes: Vec<Fish> = Vec::new();
    for entry in input.split(",")
    {
        let num = entry.parse::<usize>();

        match num
        {
            Ok(days_left) =>
            {
                fishes.push( Fish{ days_left } );
            },
            Err(_) => continue
        }
    }

    return fishes;
}

fn calculate_fish_population_after_n_days(mut fishes: Vec<Fish>, num_days: usize) -> usize
{
    for _ in 0..num_days
    {
        let mut new_fishes: Vec<Fish> = Vec::new();
        for fish in &mut fishes
        {
            let new_fish = fish.advance_day();

            match new_fish
            {
                Some(f) => new_fishes.push(f),
                None    => continue
            }
        }

        fishes.append(&mut new_fishes);
    }

    return fishes.len();
}

struct Fish
{
    days_left: usize
}

impl Fish
{
    pub fn advance_day(&mut self) -> Option<Fish>
    {
        if self.days_left == 0
        {
            self.days_left = 6;
            return Some( Fish{days_left: 8} );
        }

        self.days_left -= 1;
        return None;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_create_fishes_from_file()
    {
        let fishes = create_fishes_from_file("src/inputs/day_6_example.txt");
        assert_eq!(fishes.len(), 5);
    }

    #[test]
    fn test_first_example()
    {
        let fishes = create_fishes_from_file("src/inputs/day_6_example.txt");
        assert_eq!(calculate_fish_population_after_n_days(fishes, 80), 5934);
    }
}