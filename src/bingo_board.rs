#[derive(Default)]
pub struct BingoBoard
{
    cells: [BingoCell; 25]
}

#[derive(Default, Clone, Copy)]
struct BingoCell
{
    value: i32,
    is_marked: bool
}

impl BingoBoard
{
    pub fn set_cell_at(&mut self, x: usize, y: usize, val: i32)
    {
        self.cells[x * 5 + y] = BingoCell{value: val, is_marked: false};
    }

    pub fn get_unmarked_sum(&self) -> i32
    {
        let mut sum = 0;
        for cell in self.cells
        {
            if !cell.is_marked { sum += cell.value; }
        }

        return sum;
    }

    pub fn check_and_mark(&mut self, number: i32) -> bool
    {
        for i in 0..self.cells.len()
        {
            if self.cells[i].value == number
            {
                self.cells[i].is_marked = true;
                if self.check_bingo(i)
                {
                    // BINGO!
                    return true;
                }
            }
        }

        return false;
    }

    fn check_bingo(&self, cell_idx: usize) -> bool
    {
        let x = cell_idx / 5;
        let y = cell_idx - x * 5;

        return self.check_bingo_row(y) ||
               self.check_bingo_col(x);
    }

    fn check_bingo_col(&self, col_idx: usize) -> bool
    {
        return self.cells[col_idx*5+0].is_marked &&
               self.cells[col_idx*5+1].is_marked &&
               self.cells[col_idx*5+2].is_marked &&
               self.cells[col_idx*5+3].is_marked &&
               self.cells[col_idx*5+4].is_marked;
    }

    fn check_bingo_row(&self, row_idx: usize) -> bool
    {
        return self.cells[0*5+row_idx].is_marked &&
               self.cells[1*5+row_idx].is_marked &&
               self.cells[2*5+row_idx].is_marked &&
               self.cells[3*5+row_idx].is_marked &&
               self.cells[4*5+row_idx].is_marked;
    }

    pub fn print(&self)
    {
        for x in 0..5
        {
            println!("{}{} {}{} {}{} {}{} {}{}",
                     self.cells[0*5+x].value,
                     if self.cells[0*5+x].is_marked {"!"} else {""},

                     self.cells[1*5+x].value,
                     if self.cells[1*5+x].is_marked {"!"} else {""},

                     self.cells[2*5+x].value,
                     if self.cells[2*5+x].is_marked {"!"} else {""},

                     self.cells[3*5+x].value,
                     if self.cells[3*5+x].is_marked {"!"} else {""},

                     self.cells[4*5+x].value,
                     if self.cells[4*5+x].is_marked {"!"} else {""});
        }
    }
}