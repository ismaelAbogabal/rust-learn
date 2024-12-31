use std::{
    collections::{HashMap, HashSet},
    i32, usize,
};

pub fn test() {
    println!(
        "{}",
        min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );

    println!("{}", min_window("a".to_string(), "a".to_string()));

    println!("{}", min_window("a".to_string(), "aa".to_string()));
}

pub fn min_window(s: String, t: String) -> String {
    let mut min = "".to_string();
    let mut min_len = usize::MAX;

    let mut i = 0;
    let mut j = i;

    let chars = s.chars().collect::<Vec<char>>();

    let search_items = t.chars().collect::<HashSet<char>>();
    let mut remaining = hash_search(t);
    let mut over_found = HashMap::new() as HashMap<char, usize>;

    while j < s.len() || remaining.is_empty() {
        let len = j - i + 1;
        if remaining.is_empty() {
            if len < min_len {
                min_len = len;
                min = s[i..j].to_string();
            }

            let first_char = chars[i];

            let in_search = search_items.get(&first_char);
            if in_search != None {
                let over = over_found.get(&first_char).unwrap_or(&0);
                if *over == 0 {
                    remaining.insert(first_char, 1);
                } else {
                    over_found.insert(first_char, over - 1);
                }
            }
            i += 1;
            continue;
        }

        if remaining.is_empty() || len > min_len {
            //
        }

        let char = chars[j];
        if search_items.get(&char) == None {
            j += 1;
            continue;
        } else {
            let rem = remaining.get(&char).unwrap_or(&0);

            if *rem == 0 {
                let found = over_found.get(&char).unwrap_or(&0);

                over_found.insert(char, found + 1);
                j += 1;
            } else if *rem == 1 {
                remaining.remove(&char);
                j += 1;
            } else {
                remaining.insert(char, rem - 1);
                j += 1;
            }
        }
    }

    min
}

pub fn hash_search(t: String) -> HashMap<char, usize> {
    let mut map = HashMap::new() as HashMap<char, usize>;

    for char in t.chars() {
        let count = map.get(&char).unwrap_or(&0);

        map.insert(char, count + 1);
    }

    map
}

pub fn min_window_2(s: String, t: String) -> String {
    let mut min = "".to_string();
    let mut min_len = usize::MAX;

    let chars = s.chars().collect::<Vec<char>>();
    let mut map = HashMap::new() as HashMap<char, i32>;

    for char in t.chars() {
        let count = map.get(&char).unwrap_or(&0);

        map.insert(char, count + 1);
    }

    let mut i = 0;

    while i < s.len() {
        let mut remaining = map.clone();

        for j in i..s.len() {
            let len = j - i + 1;

            if len > min_len {
                break;
            }

            let current = chars[j];

            match remaining.get(&current) {
                Some(val) => {
                    if *val > 1 {
                        remaining.insert(current, val - 1);
                    } else {
                        remaining.remove(&current);

                        if remaining.is_empty() {
                            min_len = len;
                            min = s[i..=j].to_string();
                        }
                    }
                    //todo
                }
                None => {
                    // the first searching item isnot on the map
                    if j == i {
                        break;
                    }

                    // else continue
                }
            }
        }

        i += 1;
    }

    min
}
