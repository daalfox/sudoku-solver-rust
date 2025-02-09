#![allow(dead_code)]

type Sudoku = [[u8; 9]; 9];
type CellAddr = (usize, usize);

fn solve(table: &Sudoku) -> Sudoku {
    let t = *table;
    for (i, row) in table.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != 0 {
                continue;
            }

            let possibilites = calculate_possibilities(table, (i, j));

            dbg!(possibilites);
        }
    }

    t
}

fn calculate_possibilities(table: &[[u8; 9]; 9], cell: CellAddr) -> Vec<u8> {
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
    let r = cell.0 / 3;
    let c = cell.1 / 3;

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
    fn calculates_possibilities_removes_row_numbers() {
        let table = get_table();
        let row_index = 0;
        let got = calculate_possibilities(&table, (row_index, 2));
        let row_numbers: Vec<u8> = table[row_index].into_iter().filter(|x| *x != 0).collect();

        assert!(got.iter().all(|x| !row_numbers.contains(x)))
    }

    #[test]
    fn calculates_possibilities_removes_col_numbers() {
        let table = get_table();
        let col_index = 2;
        let got = calculate_possibilities(&table, (0, col_index));
        let col_numbers: Vec<u8> = get_col_numbers(&table, col_index);

        assert!(got.iter().all(|x| !col_numbers.contains(x)))
    }

    #[test]
    fn calculates_possibilities_removes_subgrid_numbers() {
        let table = get_table();
        let cell_addr = (0, 2);
        let got = calculate_possibilities(&table, cell_addr);
        let col_numbers: Vec<u8> = get_subgrid_numbers(&table, cell_addr);

        assert!(got.iter().all(|x| !col_numbers.contains(x)))
    }
}
