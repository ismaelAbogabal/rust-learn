use std::{
    cell,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn main() {
    let items = [
        0, 7, 0, 0, 0, 4, 0, 0, 0, 0, 0, 6, 2, 0, 0, 0, 3, 9, 0, 4, 0, 6, 5, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 0, 0, 0, 8, 0, 0, 7, 0, 6, 0, 3, 0, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 3, 7,
        0, 8, 0, 8, 1, 0, 0, 0, 6, 2, 0, 0, 0, 0, 0, 5, 0, 0, 0, 4, 0,
    ];
    let mut sudoku = BTree::sudoku(items);

    let solution = sudoku.solve_queue();

    if let Some(solution) = solution {
        solution.print_table();
    } else {
        print!("No solution");
    }

    //
}

#[derive(Debug, Clone)]
struct BTree {
    nodes: HashMap<i32, i32>,
    connections: HashMap<i32, Vec<i32>>,
}

impl BTree {
    pub fn sudoku(node_fill: [i32; 81]) -> BTree {
        let connections = BTree::connections();
        let mut nodes: HashMap<i32, i32> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let key = BTree::cell_key(r, c);
                nodes.insert(key, node_fill[key as usize]);
            }
        }

        BTree { connections, nodes }
    }

    pub fn connections() -> HashMap<i32, Vec<i32>> {
        let mut connections: HashMap<i32, Vec<i32>> = HashMap::new();
        for r in 0..9 {
            for c in 0..9 {
                let mut cell_connectsions: Vec<i32> = Vec::new();
                // get column connectsions
                for i in 0..9 {
                    if i != r {
                        cell_connectsions.push(BTree::cell_key(i, c));
                    }

                    if i != c {
                        cell_connectsions.push(BTree::cell_key(r, i));
                    }

                    let sub_col = i / 3;
                    let sub_row = i % 3;

                    let sub_grid_row = r / 3 * 3 + sub_row;
                    let sub_grid_col = c / 3 * 3 + sub_col;

                    if sub_grid_row != r || sub_grid_col != c {
                        cell_connectsions.push(BTree::cell_key(sub_grid_row, sub_grid_col));
                    }
                }

                let cell_key = BTree::cell_key(r, c);
                connections.insert(cell_key, cell_connectsions);
            }
        }

        connections
    }

    pub fn cell_key(row: i32, col: i32) -> i32 {
        row * 9 + col
    }

    pub fn cell_string(key: &i32) -> String {
        format!("{},{}", key / 9, key % 9)
    }

    pub fn cell_options(&self, cell: i32) -> Vec<i32> {
        if cell >= 81 {
            panic!("Invalid cell option");
        }

        let cellValue = self.nodes.get(&cell).unwrap();

        if *cellValue != 0 {
            return Vec::from([*cellValue]);
        }

        let connections = self.connections.get(&cell).unwrap();

        let mut set: HashSet<i32> = HashSet::new();
        for con in connections {
            let val = self.nodes.get(con).unwrap_or(&0);

            if *val != 0 {
                set.insert(*val);
            }
        }

        let mut option: Vec<i32> = Vec::new();

        for i in 1..10 {
            if !set.contains(&i) {
                option.push(i);
            }
        }

        option
    }
    pub fn solve_queue(&self) -> Option<BTree> {
        let mut trial_queue: Vec<BTree> = Vec::from([self.clone()]);

        while let Some(mut solution) = trial_queue.pop() {
            // let mut solution = active;
            let mut i = 0;
            // cell -> options
            let mut least_cell = (-1, usize::MAX);

            while i < 81 {
                let cell = solution.nodes.get(&i).unwrap();

                if *cell == 0 {
                    let options = solution.cell_options(i);

                    if options.len() == 0 {
                        break;
                    } else if options.len() == 1 {
                        // got a solution for a cell
                        solution.nodes.insert(i, options[0]);

                        i = 0;
                        continue;
                    } else {
                        if options.len() < least_cell.1 {
                            least_cell = (i, options.len());
                        }
                        // this cell is missing
                        //            solved = Some(*cell);
                    }
                }

                // go to the next tile
                i += 1;
            }

            if least_cell.0 == -1 {
                return Some(solution);
            } else {
                let options = solution.cell_options(least_cell.0);

                for opt in options {
                    let mut trial = solution.clone();
                    trial.nodes.insert(least_cell.0, opt);

                    trial_queue.push(trial);
                }
            }
        }

        None
        //todo solve the easy tiles
    }

    pub fn solve(&mut self) -> Vec<BTree> {
        let mut solution = self.clone();

        let mut i = 0;

        /// captrure remaint cell
        let mut solved: Option<i32> = None;

        while i < 81 {
            let cell = solution.nodes.get(&i).unwrap();

            if *cell == 0 {
                let options = solution.cell_options(i);

                if options.len() == 0 {
                    return Vec::new();
                } else if options.len() == 1 {
                    solution.nodes.insert(i, options[0]);

                    i = 0;
                    continue;
                } else {
                    // this cell is missing
                    solved = Some(*cell);
                }
            }

            i += 1;
        }

        match solved {
            Some(cell) => {
                let optoins = solution.cell_options(cell);

                let mut solutions: Vec<BTree> = Vec::new();
                for sub_solution in optoins {
                    let mut copy = solution.clone();
                    copy.nodes.insert(cell, sub_solution);
                    let solve = copy.solve();

                    if solve.len() > 1 {
                        panic!("More than one solution");
                    }

                    solutions.extend(solve);
                    //                    copy.solve();
                }
                solutions
            }

            None => Vec::from([solution]),
        }
    }

    pub fn print_table(&self) {
        for i in 0..81 {
            // Print a value or placeholder for empty cells
            if let Some(value) = self.nodes.get(&i) {
                print!(" {:2} ", value);
            } else {
                print!("  . "); // Use `.` for empty cells
            }

            // Add vertical separators for grid columns
            if (i + 1) % 3 == 0 && (i + 1) % 9 != 0 {
                print!("|");
            }

            // Add a newline for each row and horizontal separators for grid rows
            if (i + 1) % 9 == 0 {
                println!();
                if (i + 1) % 27 == 0 && i + 1 < 81 {
                    println!("------------+------------+-----------");
                }
            }
        }
    }
    //let key
}
