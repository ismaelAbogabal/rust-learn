pub fn test() {
    let input = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];

    let mut v = input.iter().map(|v| Vec::from(v)).collect();

    set_zeroes(&mut v);

    println!("{v:?}");
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut first_row = false;
    let mut first_col = false;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let current = matrix[row][col];

            if current != 0 {
                continue;
            }

            if row != 0 && col != 0 {
                matrix[row][0] = 0;
                matrix[0][col] = 0;
            } else {
                if row == 0 {
                    first_row = true;
                }
                if col == 0 {
                    first_col = true;
                }
            }
        }
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if (row == 0 && first_row)
                || (col == 0 && first_col)
                || matrix[row][0] == 0
                || matrix[0][col] == 0
            {
                matrix[row][col] = 0;
            }
        }
    }
}
