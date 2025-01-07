use std::{collections::HashSet, vec};

pub fn test() {
    let a = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
        [17, 18, 19, 20],
        [21, 22, 23, 24],
    ];

    let a = a.iter().map(|v| Vec::from(v)).collect::<Vec<Vec<i32>>>();

    let out = spiral_order(a);

    println!("{:?}", out);
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // we will save the direction for movements
    // we will save the boundry

    // going up will takle the y axes
    // otherwise otherwise

    let mut out = Vec::new();

    let m = matrix.len();
    let n = matrix[0].len();

    let total = m as usize * n as usize;

    let mut visited = HashSet::new() as HashSet<i32>;

    let directions = vec![1, n as i32, -1, n as i32 * -1];
    let mut mi = 0;

    let mut index = 0;

    while out.len() < total {
        let i = index / n;
        let j = index % n;

        let current = matrix[i][j];

        // we need to detect out of contenxt of visited item

        let d = directions[mi % 4];

        let di = d / n as i32;
        let dj = d % n as i32;

        let ni = i as i32 + di;
        let nj = j as i32 + dj;
    }

    out
}
