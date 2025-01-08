use std::{
    iter,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    sender: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let workers = iter::repeat_with(|| Worker::new(0, receiver.clone()))
            .take(_size)
            .collect();

        ThreadPool { sender, workers }
    }

    pub fn handle(&self, unwrap: impl FnOnce() + Send + 'static) {
        self.sender.send(Box::new(unwrap)).unwrap();
    }
}

/// listen to receive message and handle the connection
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}
//receiver: Arc<Mutex<Receiver<_>>>,

impl Worker {
    pub fn new<T>(id: usize, receiver: Arc<Mutex<Receiver<T>>>) -> Self
    where
        T: FnOnce() + Send + 'static,
    {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let fun = receiver.lock().unwrap().recv().unwrap();

                fun();
            }),
        }
    }
}
