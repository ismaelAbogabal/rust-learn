use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let m = Arc::new(Mutex::new(0));

    let threads: Vec<_> = (0..10)
        .map(|i| {
            let rc = Arc::clone(&m);
            thread::spawn(move || {
                let mut val = rc.lock().unwrap();
                *val += 1;
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
    println!("m = {m:?}");

    //thread::sleep(Duration::from_secs(3));
}
