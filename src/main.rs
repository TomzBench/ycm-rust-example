use std::sync::{Arc, Mutex};

struct Thing {
    n: u32,
}

impl Thing {
    pub fn new(n: u32) -> Thing {
        Thing { n }
    }
}

fn completeme(thing: Arc<Mutex<Thing>>) -> bool {
    let guard = thing.lock().unwrap(); // Seems OK, but lots of noise in this completion (excessive results)
    // Here, guard.n is valid code, but n is not part of completion set
    match guard.n {
        42 => true,
        _ => false,
    }
}

fn main() {
    let thing = Arc::new(Mutex::new(Thing::new(42)));

    completeme(thing);
}
