use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum State {
    W1,
    W2,
    W3,
}

#[derive(Debug)]
struct Object {
    state: State,
}

impl Object {
    fn new() -> Self {
        Object { state: State::W1 }
    }

    fn next_state(&mut self) {
        self.state = match self.state {
            State::W1 => State::W2,
            State::W2 => State::W3,
            State::W3 => State::W1, // or handle as needed
        };
    }
}

fn worker(pool: Arc<Mutex<VecDeque<Object>>>, id: usize) {
    loop {
        let mut queue = pool.lock().unwrap();
        if let Some(mut obj) = queue.pop_front() {
            println!("Worker {} processing object with state {:?}", id, obj.state);
            obj.next_state();
            println!("Worker {} changed object state to {:?}", id, obj.state);
            queue.push_back(obj);
        }
        drop(queue);
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let pool1 = Arc::new(Mutex::new(VecDeque::new()));
    let pool2 = Arc::new(Mutex::new(VecDeque::new()));
    let pool3 = Arc::new(Mutex::new(VecDeque::new()));

    for _ in 0..5 {
        pool1.lock().unwrap().push_back(Object::new());
    }

    let pool1_clone = Arc::clone(&pool1);
    let pool2_clone = Arc::clone(&pool2);
    let pool3_clone = Arc::clone(&pool3);

    let handle1 = thread::spawn(move || worker(pool1_clone, 1));
    let handle2 = thread::spawn(move || worker(pool2_clone, 2));
    let handle3 = thread::spawn(move || worker(pool3_clone, 3));

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}