#![allow(dead_code)]

use std::collections::HashMap;

type Sudoku = [[u8; 9]; 9];
type CellAddr = (usize, usize);

struct Assumption {
    alternatives: Vec<u8>,
}

fn solve(table: &Sudoku) -> Sudoku {
    let mut t = *table;

    let mut possibilities = calculate_possibilities(&t);

    // if there is a single option item in `empty_cells_map`, fill cell with that value and
    // recalculate possibilities for relative cells
    while !possibilities.is_empty() {
        match possibilities.iter().find(|cell| cell.1.len() == 1) {
            Some((addr, p)) => {
                t[addr.0][addr.1] = p[0];
            }
            // guess cells with least amount of possibilities
            None => {
                eprintln!("{possibilities:?}");
                unimplemented!("sudoku with uncertain cells are not supported");
            }
        }

        possibilities = calculate_possibilities(&t);
    }

    t
}

fn calculate_possibilities(table: &Sudoku) -> HashMap<CellAddr, Vec<u8>> {
    let mut possibilities = HashMap::new();

    for (i, row) in table.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != 0 {
                continue;
            }

            let possibilites = calculate_possibilities_for_cell(table, (i, j));

            possibilities.insert((i, j), possibilites);
        }
    }

    possibilities
}

fn calculate_possibilities_for_cell(table: &[[u8; 9]; 9], cell: CellAddr) -> Vec<u8> {
    let mut possibilities = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    possibilities.retain(|p| !table[cell.0].contains(p));

    possibilities.retain(|p| {
        let col: Vec<u8> = get_col_numbers(table, cell.1);
        !col.contains(p)
    });

    possibilities.retain(|p| {
        let subgrid: Vec<u8> = get_subgrid_numbers(table, cell);
        !subgrid.contains(p)
    });

    possibilities
}

fn get_subgrid_numbers(table: &Sudoku, cell: CellAddr) -> Vec<u8> {
    let r = (cell.0 / 3) * 3;
    let c = (cell.1 / 3) * 3;

    vec![
        table[r][c],
        table[r][c + 1],
        table[r][c + 2],
        table[r + 1][c],
        table[r + 1][c + 1],
        table[r + 1][c + 2],
        table[r + 2][c],
        table[r + 2][c + 1],
        table[r + 2][c + 2],
    ]
    .into_iter()
    .filter(|x| *x != 0)
    .collect()
}

fn get_col_numbers(table: &Sudoku, col: usize) -> Vec<u8> {
    table
        .iter()
        .filter_map(|row| {
            if row[col] != 0 {
                return Some(row[col]);
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_table() -> Sudoku {
        [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
    }

    #[test]
    fn acceptance() {
        let got = solve(&get_table());
        let want = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        assert_eq!(got, want)
    }

    #[test]
    fn acceptance2() {
        let got = solve(&[
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ]);
        let want = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert_eq!(got, want)
    }

    #[test]
    fn calculates_possibilities_removes_row_numbers() {
        let table = get_table();
        let row_index = 0;
        let got = calculate_possibilities_for_cell(&table, (row_index, 2));
        let row_numbers: Vec<u8> = table[row_index].into_iter().filter(|x| *x != 0).collect();

        assert!(got.iter().all(|x| !row_numbers.contains(x)))
    }

    #[test]
    fn calculates_possibilities_removes_col_numbers() {
        let table = get_table();
        let col_index = 2;
        let got = calculate_possibilities_for_cell(&table, (0, col_index));
        let col_numbers: Vec<u8> = get_col_numbers(&table, col_index);

        assert!(got.iter().all(|x| !col_numbers.contains(x)))
    }

    #[test]
    fn calculates_possibilities_removes_subgrid_numbers() {
        let table = get_table();
        let cell_addr = (0, 2);
        let got = calculate_possibilities_for_cell(&table, cell_addr);
        let col_numbers: Vec<u8> = get_subgrid_numbers(&table, cell_addr);

        assert!(got.iter().all(|x| !col_numbers.contains(x)))
    }
}
