use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
#[derive(Debug, Clone, Default)]
struct BingoBoard {
    row_count: [u32; 5],
    col_count: [u32; 5],
    grid: [[i32; 5]; 5],
    reverse_idx_number: HashMap<i32, Vec<(usize, usize)>>,
}

impl BingoBoard {
    fn create(grid: [[i32; 5]; 5]) -> Self {
        let mut reverse_idx_number = HashMap::<i32, Vec<(usize, usize)>>::new();
        // populate reverse idx
        for (row_nr, row) in grid.iter().enumerate() {
            for (col_nr, col) in row.iter().enumerate() {
                if let Some(v) = reverse_idx_number.get_mut(col) {
                    v.push((row_nr, col_nr));
                } else {
                    reverse_idx_number.insert(*col,vec![(row_nr, col_nr)]);
                }
            }
        }
        BingoBoard {
            row_count: Default::default(),
            col_count: Default::default(),
            grid,
            reverse_idx_number
        }
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        return self.grid[x][y] < 0
    }

    fn is_bingo(&self) -> bool {
        self.row_count.into_iter().any(|c| c == 5)
            || self.col_count.into_iter().any(|c| c == 5)
    }

    fn mark(&mut self, nr: i32) {
        if let Some(v) = self.reverse_idx_number.get(&nr)  {
            if v.len() < 1 {
                return;
            }
            for (x, y) in v.iter().copied() {
                if self.is_marked(x, y) {
                    return;
                }

                self.grid[x][y] = -self.grid[x][y];
                self.row_count[x] += 1;
                self.col_count[y] += 1;
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for rows in self.grid {
            for col in rows {
                if col >= 0 {
                    sum += col;
                }
            }
        }
        sum
    }
}

fn parse(path: &Path) -> Result<(Vec<i32>, Vec<BingoBoard>)> {
    let mut numbers = Vec::new();
    let mut boards = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    // parse bingo numbers
    {
        let number_line = lines.next().unwrap()?;
        for number in number_line.split(",") {
            numbers.push(number.parse()?);
        }
    }
    loop {
        if lines.next().is_none() {
            break;
        }

        let mut grid: [[i32; 5]; 5] = [[0; 5]; 5];
        for r in 0..5 {
            let row = lines.next().unwrap()?;
            let (a, b, c, d, e) = scan_fmt::scan_fmt!(&row, "{} {} {} {} {}", i32, i32, i32, i32, i32)?;
            grid[r][0] = a;
            grid[r][1] = b;
            grid[r][2] = c;
            grid[r][3] = d;
            grid[r][4] = e;
        }

        boards.push(BingoBoard::create(grid));
    }
    Ok((numbers, boards))
}

fn main() -> Result<()> {
    let (numbers, mut boards) = parse(&Path::new("input.txt"))?;
    let mut bingo_count = 0;
    let board_count = boards.len();
    for number in numbers {
        for board in &mut boards {
            if board.is_bingo() {
                continue;
            }

            board.mark(number);

            if board.is_bingo() {
                bingo_count += 1;
            }

            if bingo_count == board_count {
                println!("At number: {} Answer: {}", number, number * board.sum_unmarked());
                return Ok(());
            }
        }
    }
    Ok(())
}
