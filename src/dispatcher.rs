use std::sync::Mutex;

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


pub struct Dispatcher<'a, E: Event> {
    id: usize,
    sequencer: Option<&'a Dispatcher<'a, SequenceEvent>>,
    queue_map_mutex: Mutex<QueueMap<usize, E>>
}

unsafe impl<'a, E: Event> Sync for Dispatcher<'a, E> {}

impl<'a, E: Event> Dispatcher<'a, E> {
    pub fn new<'b>() -> Dispatcher<'b, E> {
        Dispatcher {
            id: id_counter::next_id(),
            sequencer: None,
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

pub trait HasDispatcher<E: Event> {
    fn dispatcher<'a>(&'a self) -> &'a Dispatcher<'a, E>;
}

pub trait Broadcaster<E: Event> {
    fn broadcast(&self, event: E);
}

impl<E: Event> Broadcaster<E> for HasDispatcher<E> {
    fn broadcast(&self, event: E) {
        self.dispatcher().broadcast(event);
    }
}

impl<'a, E: Event> Broadcaster<E> for Dispatcher<'a, E> {
    fn broadcast(&self, event: E) {
        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        (*queue_map).push(event);

        self.sequencer.map(|sequencer| {
            sequencer.broadcast(SequenceEvent::new(self.id));
        });
    }
}

