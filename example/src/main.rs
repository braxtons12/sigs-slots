extern crate sigs_slots;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;

use sigs_slots::{Event, Signal, SyncSignal, Slot };

struct Thing1 {
    slots: Vec<Rc<RefCell<dyn Slot<String>>>>,
}

impl Thing1 {
    pub fn new() -> Thing1 {
        Thing1 { slots: vec![] }
    }
}

impl Signal<String> for Thing1 {
    fn connect(&mut self, slot: Rc<RefCell<dyn Slot<String>>>) {
        self.slots.push(slot);
    }

    fn emit(&mut self, event: Event<String>) {
        for slot in &self.slots {

            slot.borrow_mut().consume(&event);
        }
    }
}

struct Thing2 {
    pub message: String
}

impl Thing2 {
    pub fn new(message: String) -> Thing2 {
        Thing2 { message }
    }
}

impl Slot<String> for Thing2 {
    fn consume(&mut self, event: &Event<String>) {
        match event {
            Event::Sig(x) => println!("{}", x),
            _ => println!("Signal wasn't a string!")
        }
    }
}

struct Thing3 {
    slots: Vec<Arc<RwLock<dyn Slot<String>>>>
}

impl Thing3 {
    pub fn new() -> Thing3 {
        Thing3 { slots: vec![] }
    }
}

impl SyncSignal<String> for Thing3 {
    fn connect(&mut self, slot: Arc<RwLock<dyn Slot<String>>>) {
        self.slots.push(slot);
    }

    fn emit(&mut self, event: Event<String>) {
        for slot in &self.slots {
            slot.write().unwrap().consume(&event);
        }
    }
}

unsafe impl Send for Thing3 {

}

fn main() {

    let mut thing1 = Thing1::new();
    let mut thing2 = Thing2::new("Hello from thing2!".to_string());
    let mut thing4 = Thing2::new("Hello from thing4!".to_string());

    let thing2rc = Rc::new(RefCell::new(thing2));
    let thing4arc = Arc::new(RwLock::new(thing4));
    let mut thing3 = Thing3::new();
    thing3.connect(thing4arc.clone());
    let thread2 = std::thread::spawn(move  || {
        thing3.emit(Event::Sig("Hello from thing3!".to_string()));
    });
    thing1.connect(thing2rc.clone());
    thing1.emit(Event::Sig("Hello from thing1!".to_string()));
    println!("{}", thing2rc.borrow_mut().message);
    println!("{}", thing4arc.read().unwrap().message);
    thread2.join();
}
