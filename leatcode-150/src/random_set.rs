use std::collections::{HashMap, HashSet};

use rand::Rng;

pub fn test() {
    let mut set = RandomizedSet::new();

    set.insert(0);
    set.remove(0);
}

#[derive(Debug)]
struct RandomizedSet {
    map: HashMap<i32, i32>,
    list: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        // check that number isnt already there
        //
        let old = self.map.get(&val);

        if let Some(_) = old {
            false
        } else {
            self.list.push(val);
            self.map.insert(val, self.list.len() as i32 - 1);

            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        let old = self.map.get(&val);

        if let Some(old) = old {
            //swap with last item
            //
            let len = self.list.len();
            self.list.swap(*old as usize, len - 1);

            self.list.pop();

            if *old as usize != len - 1 {
                let swapped_item = self.list[*old as usize];
                self.map.insert(swapped_item, *old);
            }
            self.map.remove(&val);

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let num = rand::thread_rng().gen_range(0..self.list.len());

        self.list[num]
    }
}
