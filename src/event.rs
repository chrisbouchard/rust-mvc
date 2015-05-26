use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

trait Event : Copy {}

struct Dispatcher<E: Event> {
    inboxes: HashMap<usize, VecDeque<E>>
}

impl<E: Event> Dispatcher<E> {
    fn new() {
        Dispatcher { inboxes: vec![] }
    }

    fn register(&mut self, id: usize) {
        // TODO: Implement this method.
        0
    }

    fn broadcast(&mut self, event: E) {
        // TODO: Implement this method.
    }

    fn read(&mut self, id: usize) -> Option<Event> {
        // TODO: Implement this method.
        None
    }

    fn time(&self, id: usize) -> Option<Timespec> {
        // TODO: Implement this method.
        None
    }
}

trait HasDispatcher<E: Event> {
    fn with_dispatcher<F: FnOnce<&mut Dispatcher<E>>>(&self, action: F) -> F::Output;
}

trait Broadcaster<E: Event> {
    fn broadcast(&self, event: E);
}

impl<E: Event, T: HasDispatcher<E>> Broadcaster<E> for HasDispatcher<E> {
    fn broadcast(&self, event: E) {
        self.with_dispatcher(|dispatcher| dispatcher.broadcast(event))
    }
}

trait Handler<E: Event> {
    type Application;
    type Context;

    fn handle(&self, event: &E, application: &Application, context: &mut Context);
}

struct Controller<'a, A, C> {
    id: usize,
    application: &'a A,
    context: C,
    timers: Vec<Fn<&mut Controller<'a, A, C>>>,
    updaters: Vec<Fn<&mut Controller<'a, A, C>>>
}

impl<'a, E: Event, A: HasDispatcher<E>> HasDispatcher<E> for Controller<'a, A> {
    fn with_dispatcher(&self, action: FnOnce<&mut Dispatcher<E>>) {
        self.parent.with_dispatcher(action)
    }
}

trait GenericHandler<A, C> {
    fn time<'a>(&self, controller: &mut Controller<'a, A, C>);
    fn update<'a>(&self, controller: &mut Controller<'a, A, C>);
}

impl<E: Event, A: HasDispatcher<E>, C> GenericHandler<A, C> for Handler<E, Application=A, Context=C> {
    fn time<'a>(&self, controller: &mut Controller<'a, A, C>) {
        (controller.application as &HasDispatcher<E>).with_dispatcher(|dispatcher| {
            dispatcher.time(controller.id)
        })
    }
}

trait AcceptsHandler<E: Event> {
    fn add(&mut self, handler: Handler<E>);
}

impl<'a, E: Event, A: HasDispatcher<E>, C> AcceptsHandler<E> for Controller<'a, A, C> {
    fn add(&mut self, handler: Handler<E>) {
        self.with_dispatcher(|dispatcher| {
            dispatcher.register(self.id);
        });

        updaters.push(|controller| {
            controller.with_dispatcher(|ctrl_dispatcher| {
                match ctrl_dispatcher.read(controller.id) {
                    None => (),
                    Some(event) =>
                        handler.handle(&event, controller.application, &mut controller.context)
                }
            })
        })
    }
}

