use super::dispatcher::{Dispatcher, HasDispatcher, SequenceEvent};

pub trait Application : HasDispatcher<SequenceEvent> + Sync {
    fn sequencer<'a>(&'a self) -> &'a Dispatcher<'a, SequenceEvent> {
        self.dispatcher()
    }
}

