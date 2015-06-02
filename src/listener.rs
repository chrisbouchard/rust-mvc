use std::collections::HashMap;
use std::sync::Arc;

use super::id_counter;

use super::application::*;
use super::dispatcher::*;
use super::event::*;

pub struct Listener<'a, A: Application + 'a, C> {
    id: usize,
    application: Arc<A>,
    context: C,
    sequencer: Arc<Sequencer>,
    updaters: HashMap<usize, Box<Fn(&A, &mut C) + 'a>>
}

impl<'a, A: Application + 'a, C> Listener<'a, A, C> {
    pub fn new<'b>(application: Arc<A>, context: C, sequencer: Arc<Sequencer>) -> Listener<'b, A, C> {
        let listener_id = id_counter::next_id();

        info!("Creating new listener with id {} and sequencer {}", listener_id, sequencer.id());

        Listener {
            id: listener_id,
            application: application,
            context: context,
            sequencer: sequencer,
            updaters: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        info!("Starting main loop for listener {}", self.id);

        self.sequencer.register(self.id);

        loop {
            if let Some(seq_event) = self.sequencer.receive(self.id) {
                info!("Listener {} got a sequence event from sequencer {}: {:?}", self.id, self.sequencer.id(), seq_event);

                let dispatcher_id = seq_event.dispatcher_id();

                if let Some(update_box) = self.updaters.get(&dispatcher_id) {
                    info!("Calling updater for dispatcher {}", dispatcher_id);
                    (*update_box)(&*self.application, &mut self.context)
                }
            }
        }
    }
}


pub trait Handler<E: Event> {
    type Application: Application;
    type Context;

    fn handle(&self, event: E, app: &Self::Application, context: &mut Self::Context);
}


pub trait AcceptsHandler<E: Event> {
    type Application: Application;
    type Context;

    fn add_handler<H>(&mut self, handler: H)
        where H: Handler<E, Application=Self::Application, Context=Self::Context>;
}

impl<'a, E, A, C> AcceptsHandler<E> for Listener<'a, A, C>
where E: Event + 'a, A: Application + 'a + HasDispatcher<E> {
    type Application = A;
    type Context = C;

    fn add_handler<H>(&mut self, handler: H) where H: Handler<E, Application=A, Context=C> + 'a {
        let listener_id = self.id;

        info!("Adding handler to listener {}", listener_id);

        let app: Arc<A> = self.application.clone();
        let dispatcher: Arc<Dispatcher<E>> = app.dispatcher();
        let dispatcher_id = dispatcher.id();

        debug!("Matching handler on listener {} to dispatcher {}", listener_id, dispatcher_id);

        dispatcher.register(listener_id);

        self.updaters.insert(dispatcher_id, Box::new(move |app, context| {
            dispatcher.receive(listener_id).map(|event| {
                info!("Listener {} got an event from dispatcher {}: {:?}", listener_id, dispatcher_id, event);
                handler.handle(event, app, context)
            });
        }));
    }
}

