use crate::height_map::HeightMap;

pub fn day_15_1() -> usize
{
    let map = HeightMap::new_from_file("src/inputs/day_15_input.txt");
    return find_lowest_risk_path(&map);
}

fn find_lowest_risk_path(map: &HeightMap) -> usize
{
    let start = 0;
    let end   = map.get_num_cells() - 1;

    let prev_cell_list = find_path_A_star(map, start, end);
    println!("A* finished!");
    return accumulate_path_risk(map, &prev_cell_list, start, end);
}

#[allow(non_snake_case)]
fn find_path_A_star(map: &HeightMap, start: usize, end: usize) -> Vec<Option<usize>>
{
    let mut costs_so_far = Vec::new();
    costs_so_far.resize(map.get_num_cells(), usize::MAX);
    costs_so_far[start] = 0;

    let mut prev_cell_list = Vec::new();
    prev_cell_list.resize(map.get_num_cells(), None);

    let mut candidates = vec![(start,0)]; // (index, risk)
    while !candidates.is_empty()
    {
        let i = candidates.pop().unwrap().0;

        if i == end { break; }

        // neighbour: (value: usize, index: usize)
        for n in map.get_neighbours(i)
        {
            if prev_cell_list[i].unwrap_or(usize::MAX) == n.1 { continue; }

            let new_cost = costs_so_far[i] + n.0;
            if new_cost >= costs_so_far[n.1] { continue; }

            costs_so_far[n.1] = new_cost;
            prev_cell_list[n.1] = Some(i);

            candidates.push((n.1, new_cost + end - n.1));
        }
        // Sort in descendent risk
        candidates.sort_unstable_by(|a,b|{b.cmp(&a)});
    }

    return prev_cell_list;
}

fn accumulate_path_risk(map: &HeightMap,
                        prev_cell_list: &Vec<Option<usize>>,
                        start: usize,
                        end: usize)
-> usize
{
    let mut total_risk = 0;
    let mut i = end;
    while i != start
    {
        match prev_cell_list[i]
        {
            Some(idx) =>
            {
                total_risk += map.get_cell_value(i);
                i = idx;
            }
            None => break
        }
    }
    return total_risk;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        let map = HeightMap::new_from_file("src/inputs/day_15_example.txt");
        assert_eq!(find_lowest_risk_path(&map), 40);
    }
}