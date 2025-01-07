pub fn test() {
    let mx = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut ve = &mut mx.iter().map(|a| Vec::from(a)).collect();
    println!("{ve:?}");

    rotate(&mut ve);

    println!("{ve:?}");
}

pub fn rotate(mut matrix: &mut Vec<Vec<i32>>) {
    // m == n
    // we could replace the outer circle then the inner and soo one
    // can we get the left line and add it on the top

    // we can create a function the replace a cirlce
    // cirlce count = l / n + l % 2
    let n = matrix.len();

    for i in 0..n {
        for j in 0..n {
            if i > j {
                let temp = matrix[i][j];

                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }

    matrix.into_iter().for_each(|v| {
        v.reverse();
    });
}

pub fn reverse_diagonally(mx: &mut Vec<Vec<i32>>) {
    let n = mx.len();

    for i in 0..n {
        for j in 0..n {
            if i > j {
                let temp = mx[i][j];

                mx[i][j] = mx[j][i];
                mx[j][i] = temp;
            }
        }
    }
}
