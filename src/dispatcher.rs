use std::sync::{Arc, Mutex};

use super::id_counter;

use super::event::*;
use super::queue_map::*;


#[derive(Clone, Copy, Debug)]
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
        let dispatcher_id = id_counter::next_id();

        info!("Creating new dispatcher with id {} and sequencer {}", dispatcher_id, sequencer.id);

        Dispatcher {
            id: dispatcher_id,
            sequencer: Some(sequencer),
            queue_map_mutex: Mutex::new(QueueMap::new())
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn register(&self, listener_id: usize) {
        info!("Registering listener id {} on dispatcher {}", listener_id, self.id);
        debug!("Waiting for mutex on dispatcher {}...", self.id);

        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        debug!("Got mutex on dispatcher {}!", self.id);

        queue_map.add(listener_id);

        debug!("Dispatcher {}: {:?}", self.id, *queue_map);
        debug!("Letting go of mutex on dispatcher {}", self.id);
    }

    pub fn receive(&self, listener_id: usize) -> Option<E> {
        let mut queue_map = self.queue_map_mutex.lock().unwrap();
        queue_map.pop(&listener_id)
    }

    pub fn broadcast(&self, event: E) {
        info!("Broadcasting event on dispatcher {}: {:?}", self.id, event);
        debug!("Waiting for mutex on dispatcher {}...", self.id);

        {
            let mut queue_map = self.queue_map_mutex.lock().unwrap();
            debug!("Got mutex on dispatcher {}!", self.id);

            queue_map.push(event);

            debug!("Dispatcher {}: {:?}", self.id, *queue_map);
            debug!("Letting go of mutex on dispatcher {}", self.id);
        }

        if let Some(ref sequencer) = self.sequencer {
            debug!("Broadcasting sequence event from dispatcher {} to sequencer {}", self.id, sequencer.id);
            sequencer.broadcast(SequenceEvent::new(self.id));
        }
    }
}


pub fn sequencer() -> Sequencer {
    let sequencer_id = id_counter::next_id();

    info!("Creating new sequencer with id {}", sequencer_id);

    Dispatcher {
        id: sequencer_id,
        sequencer: None,
        queue_map_mutex: Mutex::new(QueueMap::new())
    }
}


pub trait HasDispatcher<E: Event> {
    fn dispatcher(&self) -> Arc<Dispatcher<E>>;
}

