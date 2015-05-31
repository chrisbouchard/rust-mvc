use std::sync::{Arc, Mutex};

use super::id_counter;

use super::event::*;
use super::queue_map::*;


#[derive(Clone, Copy)]
pub struct SequenceEvent {
    dispatcher_id: usize
}

impl SequenceEvent {
    pub fn new(dispatcher_id: usize) -> SequenceEvent {
        SequenceEvent { dispatcher_id: dispatcher_id }
    }

    pub fn dispatcher_id(&self) -> usize {
        self.dispatcher_id
    }
}

impl Event for SequenceEvent {}


pub type Sequencer = Dispatcher<SequenceEvent>;


pub struct Dispatcher<E: Event> {
    id: usize,
    sequencer: Option<Arc<Sequencer>>,
    queue_map_mutex: Mutex<QueueMap<usize, E>>
}

unsafe impl<E: Event> Sync for Dispatcher<E> {}

impl<E: Event> Dispatcher<E> {
    pub fn new(sequencer: Arc<Sequencer>) -> Dispatcher<E> {
        Dispatcher {
            id: id_counter::next_id(),
            sequencer: Some(sequencer),
            queue_map_mutex: Mutex::new(QueueMap::new())
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn register(&self, id: usize) {
        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        (*queue_map).add(id);
    }

    pub fn receive(&self, id: usize) -> Option<E> {
        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        (*queue_map).pop(&id)
    }
}


pub fn sequencer() -> Sequencer {
    Dispatcher {
        id: id_counter::next_id(),
        sequencer: None,
        queue_map_mutex: Mutex::new(QueueMap::new())
    }
}


pub trait HasDispatcher<E: Event> {
    fn dispatcher(&self) -> Arc<Dispatcher<E>>;
}

pub trait Broadcaster<E: Event> {
    fn broadcast(&self, event: E);
}

impl<E: Event> Broadcaster<E> for HasDispatcher<E> {
    fn broadcast(&self, event: E) {
        (*self.dispatcher()).broadcast(event);
    }
}

impl<E: Event> Broadcaster<E> for Dispatcher<E> {
    fn broadcast(&self, event: E) {
        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        (*queue_map).push(event);

        self.sequencer.as_ref().map(|sequencer| {
            (*sequencer).broadcast(SequenceEvent::new(self.id));
        });
    }
}

