use std::sync::Arc;

use super::dispatcher::*;

pub trait Application : HasDispatcher<SequenceEvent> + Sync {
    fn sequencer(&self) -> Arc<Sequencer> {
        self.dispatcher()
    }
}

