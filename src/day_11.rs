use crate::input_parser::parse_input_at;

const NUM_ROWS:  usize = 10;
const NUM_COLS:  usize = 10;
const GRID_SIZE: usize = NUM_COLS * NUM_ROWS;

struct OctoGrid
{
    cells: Vec<DumboOctopus>
}

struct DumboOctopus
{
    val: usize,
    already_flashed: bool
}

pub fn day_11_1() -> usize
{
    let mut grid = OctoGrid::new_from_file("src/inputs/day_11_input.txt");
    return grid.simulate_n_steps(100);
}

impl OctoGrid
{
    pub fn print(&self)
    {
        let mut i = 0;
        for _ in 0..NUM_ROWS
        {
            for _ in 0..NUM_COLS
            {
                print!("{}", self.cells[i].val);
                i += 1;
            }
            print!("\n");
        }
    }

    pub fn new_from_file(path: &str) -> Self
    {
        let input = parse_input_at(path).unwrap_or_default();

        let mut grid = OctoGrid{ cells: Vec::with_capacity(GRID_SIZE) };
        for line in input.lines()
        {
            for c in line.chars()
            {
                let new_octopus = DumboOctopus
                {
                    val: c.to_digit(10).unwrap_or_default() as usize,
                    already_flashed: false
                };
                grid.cells.push(new_octopus);
            }
        }
        return grid;
    }

    pub fn simulate_n_steps(&mut self, num_steps: usize) -> usize
    {
        let mut flash_count = 0;
        for _ in 0..num_steps
        {
            flash_count += self.step();
        }
        return flash_count;
    }

    fn step(&mut self) -> usize
    {
        self.reset_flashes();

        let mut flash_count = 0;
        for i in 0..self.cells.len()
        {
            flash_count += self.increment_cell(i);
        }

        return flash_count;
    }

    fn increment_cell(&mut self, idx: usize) -> usize
    {
        if self.cells[idx].already_flashed { return 0; }

        self.cells[idx].val = (self.cells[idx].val + 1) % 10;
        if self.cells[idx].val == 0
        {
            return self.flash(idx);
        }
        return 0;
    }

    fn flash(&mut self, idx: usize) -> usize
    {
        if self.cells[idx].val != 0 || self.cells[idx].already_flashed
        {
            return 0;
        }

        self.cells[idx].already_flashed = true;

        let neighbour_indices = self.get_neighbours_indices(idx);

        let mut flash_count = 1;
        for n_idx in neighbour_indices
        {
            flash_count += self.increment_cell(n_idx);
        }
        return flash_count;
    }

    fn get_neighbours_indices(&self, idx: usize) -> Vec<usize>
    {
        let base = idx as i64;

        let right       = base + 1;
        let right_low   = base + (NUM_COLS as i64) + 1;
        let low         = base + (NUM_COLS as i64);
        let left_low    = base + (NUM_COLS as i64) - 1;
        let left        = base - 1;
        let left_up     = base - (NUM_COLS as i64) - 1;
        let up          = base - (NUM_COLS as i64);
        let right_up    = base - (NUM_COLS as i64) + 1;

        let y = idx / NUM_COLS;
        let x = idx - y * NUM_COLS;

        let mut neighbours = Vec::with_capacity(8);
        if x > 0
        {
            neighbours.push(left as usize);

            if left_low < GRID_SIZE as i64 && y < NUM_ROWS-1 { neighbours.push(left_low as usize); }
            if left_up >= 0 && y > 0 { neighbours.push(left_up as usize); }
        }
        if x < NUM_COLS - 1
        {
            neighbours.push(right as usize);

            if right_low < GRID_SIZE as i64 && y < NUM_ROWS-1 { neighbours.push(right_low as usize); }
            if right_up >= 0 && y > 0 { neighbours.push(right_up as usize); }
        }

        if low < GRID_SIZE as i64 && y < NUM_ROWS-1
        {
            neighbours.push(low as usize);
        }
        if up >= 0 as i64 && y > 0
        {
            neighbours.push(up as usize);
        }

        assert!(neighbours.len() != 1);
        assert!(neighbours.len() != 2);
        assert!(neighbours.len() != 4);
        assert!(neighbours.len() != 6);
        assert!(neighbours.len() != 7);
        assert!(neighbours.len() <  9);

        return neighbours;
    }

    fn reset_flashes(&mut self)
    {
        for octopus in &mut self.cells
        {
            octopus.already_flashed = false;
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
        let mut grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        assert_eq!(grid.simulate_n_steps(100), 1656);
    }

    #[test]
    fn test_step()
    {
        let mut grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        assert_eq!(grid.step(), 0);
        assert_eq!(grid.step(), 35);
    }

    #[test]
    fn test_10_steps()
    {
        let mut grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        assert_eq!(grid.simulate_n_steps(10), 204);
    }

    #[test]
    fn test_get_neighbours_indices_mid()
    {
        let grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        let neighbours = grid.get_neighbours_indices(11);

        assert_eq!(neighbours.len(), 8);
    }

    #[test]
    fn test_get_neighbours_indices_top_left_corner()
    {
        let grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        let neighbours = grid.get_neighbours_indices(0);

        assert_eq!(neighbours.len(), 3);
    }

    #[test]
    fn test_get_neighbours_indices_low_right_corner()
    {
        let grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        let neighbours = grid.get_neighbours_indices(99);

        assert_eq!(neighbours.len(), 3);
    }

    fn test_get_neighbours_indices_low_left_corner()
    {
        let grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        let neighbours = grid.get_neighbours_indices(90);

        assert_eq!(neighbours.len(), 3);
    }

    fn test_get_neighbours_indices_top_right_corner()
    {
        let grid = OctoGrid::new_from_file("src/inputs/day_11_example.txt");
        let neighbours = grid.get_neighbours_indices(9);

        assert_eq!(neighbours.len(), 3);
    }

    #[test]
    fn test_get_x_y()
    {
        let idx = 42;

        let y = idx / NUM_COLS;
        let x = idx - y * NUM_COLS;

        assert_eq!(x, 2);
        assert_eq!(y, 4);
    }
}