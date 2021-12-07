use crate::input_parser::parse_input_at;

pub fn day_6_1() -> usize
{
    let mut day_counter_slots = get_counters_from_input("src/inputs/day_6_input.txt");
    simulate_n_days(&mut day_counter_slots, 80);
    return get_counters_sum(&day_counter_slots);
}

pub fn day_6_2() -> usize
{
    let mut day_counter_slots = get_counters_from_input("src/inputs/day_6_input.txt");
    simulate_n_days(&mut day_counter_slots, 256);
    return get_counters_sum(&day_counter_slots);
}

// Slot N holds the number of fishes that have N days left to give birth.
fn get_counters_from_input(path:&str) -> [usize; 9]
{
    let mut day_counter_slots = [0,0,0,0,0,0,0,0,0];

    let input = parse_input_at(path)
                    .unwrap_or("".to_string());

    for entry in input.split(",")
    {
        match entry.parse::<usize>()
        {
            Ok(days_left) => if days_left < 7 { day_counter_slots[days_left] += 1 },
            Err(_)        => continue
        }
    }

    return day_counter_slots;
}

fn simulate_n_days(counters: &mut [usize;9], num_days: usize)
{
    for _ in 0..num_days
    {
        let tmp = counters[0];
        for i in 0..8
        {
            counters[i] = counters[i+1];
        }
        counters[6] += tmp; // Old fishes get a new Time To Give Birth of 6
        counters[8]  = tmp; // Newborns need an extra 2 days
    }
}

fn get_counters_sum(counters: &[usize; 9]) -> usize
{
    let mut count = 0;
    for slot in counters
    {
        count += slot;
    }
    return count;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_example()
    {
        let mut day_counter_slots = get_counters_from_input("src/inputs/day_6_example.txt");
        simulate_n_days(&mut day_counter_slots, 80);
        assert_eq!(get_counters_sum(&day_counter_slots), 5934);
    }

    #[test]
    fn test_second_example()
    {
        let mut day_counter_slots = get_counters_from_input("src/inputs/day_6_example.txt");
        simulate_n_days(&mut day_counter_slots, 256);
        assert_eq!(get_counters_sum(&day_counter_slots), 26984457539);
    }

    #[test]
    fn test_get_example_counters()
    {
        let counters = get_counters_from_input("src/inputs/day_6_example.txt");

        assert_eq!(counters[1], 1);
        assert_eq!(counters[2], 1);
        assert_eq!(counters[3], 2);
        assert_eq!(counters[4], 1);
    }
}