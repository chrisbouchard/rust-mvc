use super::dispatcher::{HasDispatcher, SequenceEvent};

pub trait Application : HasDispatcher<SequenceEvent> + Sync {
    fn sequencer<'a>(&'a self) -> &'a Dispatcher<'a, SequnceEvent> {
        self.dispatcher();
    }
}

