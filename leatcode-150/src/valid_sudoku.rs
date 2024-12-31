use std::collections::HashSet;

pub fn test() {
    let a = [
        ["8", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ];

    let input = a
        .iter()
        .map(|r| {
            r.iter()
                .map(|f| f.chars().next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect();

    let valid = is_valid_sudoku(input);
    println!("{valid}")
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let validation_set = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9']);

    for i in 0..9 {
        let mut row_remaining = validation_set.clone();
        let mut col_remaining = validation_set.clone();
        let mut cell_remaining = validation_set.clone();

        for j in 0..9 {
            let row_val = board[i][j];
            let col_val = board[j][i];

            let cell_row = i / 3 * 3 + j / 3;
            let cell_col = i % 3 * 3 + j % 3;
            let cell_val = board[cell_row][cell_col];

            if row_val != '.' {
                let row_ok = row_remaining.remove(&row_val);

                if !row_ok {
                    println!("{} {} row", i, j);
                    return false;
                }
            }

            if col_val != '.' {
                let col_ok = col_remaining.remove(&col_val);
                if !col_ok {
                    // println!("{} {} col", i, j);
                    return false;
                }
            }

            if cell_val != '.' {
                let cell_ok = cell_remaining.remove(&cell_val);

                // println!("{i} {j} {cell_row} {cell_col}");
                if !cell_ok {
                    // println!("{} {} cell", i, j);
                    return false;
                }
            }
        }
    }

    true
    //
}
