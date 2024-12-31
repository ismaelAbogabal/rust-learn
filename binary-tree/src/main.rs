use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

fn main() {
    // todo
    let nodes = [['w', 'x'], ['x', 'y'], ['y', 'z'], ['v', 'z']];
    let tree = BinaryTree::from(&nodes, 'a', true);

    // print!("{}", tree);
    // tree.print_rec();
    // tree.print_queue();
    //
    let path = tree.smallest_path('w', 'z');
    print!("{}", path);
}

struct BinaryTree {
    connections: HashMap<char, Vec<char>>,
    top_node: char,
}

impl BinaryTree {
    pub fn from(data: &[[char; 2]], top_node: char, twoWay: bool) -> BinaryTree {
        let mut map: HashMap<char, Vec<char>> = HashMap::new();

        for [s, d] in data {
            BinaryTree::add_connection(&mut map, s, d);

            if twoWay {
                BinaryTree::add_connection(&mut map, d, s);
            }
        }

        BinaryTree {
            top_node,
            connections: map,
        }
    }

    pub fn add_connection(map: &mut HashMap<char, Vec<char>>, src: &char, dist: &char) {
        //
        match map.get(src) {
            Some(arr) => {
                let mut vec = Vec::clone(arr);
                vec.push(*dist);

                map.insert(*src, vec);
            }
            _ => {
                map.insert(*src, Vec::from([*dist]));
            }
        }
    }

    pub fn print_queue(&self) {
        let mut queue: Vec<char> = Vec::from([self.top_node]);
        let mut visted: HashSet<char> = HashSet::new();

        while !queue.is_empty() {
            let char = queue.pop().unwrap();

            if visted.contains(&char) {
                continue;
            }
            visted.insert(char);

            print!("{} ", &char);

            let connections = self.connections.get(&char);
            match connections {
                Some(con) => {
                    for char in con {
                        queue.insert(0, *char);
                    }
                }
                None => (),
            }
        }

        print!("\n");
    }

    pub fn print_rec(&self) {
        let mut visted: HashSet<char> = HashSet::new();
        self.print_branch(&self.top_node, &mut visted);
        print!("\n");
    }

    fn print_branch(&self, node: &char, visted: &mut HashSet<char>) {
        if visted.contains(&node) {
            return;
        }

        visted.insert(*node);

        print!("{} ", node);

        if let Some(subnodes) = self.connections.get(node) {
            for n in subnodes {
                self.print_branch(n, visted);
            }
        }
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn smallest_path(&self, from: char, to: char) -> i32 {
        let mut queue = Vec::from([(from, 0 as i32)]);

        while !queue.is_empty() {
            let (current, size) = queue.pop().unwrap();
            if current == to {
                return size;
            }

            let con = self.connections.get(&current);

            if let Some(con) = con {
                for sub in con {
                    queue.insert(0, (*sub, size + 1));
                }
            }
        }

        return -1;
        //
    }
}

impl std::fmt::Display for BinaryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (src, dist) in &self.connections {
            f.write_str(&src.to_string())?;
            f.write_str(" -> ")?;
            for d in dist {
                f.write_str(&d.to_string())?;
                f.write_str(" ")?;
            }

            f.write_str("\n")?;
        }
        f.write_str("")
    }
}
