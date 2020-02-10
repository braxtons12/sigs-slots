use std::rc::Rc;
use std::cell::RefCell;

/// Generic Event Enum for transmitting a signal 
pub enum Event<T> {
    Sig(T),
}

/// Generic Signal trait to be implemented by the object emitting the signals
pub trait Signal<T> {
    /// Connects the given slot to this signal's list of consumers
    /// # Arguments
    /// * 'slot' - An Rc wrapped RefCell containing the slot to be connected
    fn connect(&mut self, slot: Rc<RefCell<dyn Slot<T>>>);
    /// Emits the given signal to all connected slots
    /// # Arguments
    /// * 'event' - the event to be sent to the slots
    fn emit(&mut self, event: Event<T>);
}

/// Generic slot trait to be implemented by the object consuming the signals
pub trait Slot<T> {
    /// Consumes the event emitted by the signal(s) this slot is connected to
    /// # Arguments
    /// * 'event' - the event this slot is consuming
    fn consume(&mut self, event: &Event<T>);
}
