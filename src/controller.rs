use std::collections::HashMap;
use std::sync::Arc;

use super::id_counter;

use super::application::*;
use super::dispatcher::*;
use super::event::*;

pub struct Controller<'a, A: Application + 'a, C> {
    id: usize,
    application: Arc<A>,
    context: C,
    sequencer: Arc<Sequencer>,
    updaters: HashMap<usize, Box<Fn(&A, &mut C) + 'a>>
}

impl<'a, A: Application + 'a, C> Controller<'a, A, C> {
    pub fn new<'b>(application: Arc<A>, context: C, sequencer: Arc<Sequencer>) -> Controller<'b, A, C> {
        Controller {
            id: id_counter::next_id(),
            application: application,
            context: context,
            sequencer: sequencer,
            updaters: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            match (*self.sequencer).receive(self.id) {
                None => (),
                Some(seq_event) => {
                    let dispatcher_id = seq_event.dispatcher_id();

                    match self.updaters.get(&dispatcher_id) {
                        None => (),
                        Some(update_box) => (*update_box)(&*self.application, &mut self.context)
                    }
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

impl<'a, E, A, C> AcceptsHandler<E> for Controller<'a, A, C>
where E: Event + 'a, A: Application + 'a + HasDispatcher<E> {
    type Application = A;
    type Context = C;

    fn add_handler<H>(&mut self, handler: H) where H: Handler<E, Application=A, Context=C> + 'a {
        let id = self.id;

        let app: Arc<A> = self.application.clone();
        let dispatcher: Arc<Dispatcher<E>> = app.dispatcher();
        let dispatcher_id = (*dispatcher).id();

        (*dispatcher).register(id);

        self.updaters.insert(dispatcher_id, Box::new(move |app, context| {
            (*dispatcher).receive(id).map(|event| handler.handle(event, app, context));
        }));
    }
}

