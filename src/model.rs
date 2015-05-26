use super::event;

struct ModelSupport<'a> {
    listeners: Vec<&'a Listener>
}

trait Model {
    fn model_support<'a>(&'a mut self) -> &'a mut ModelSupport;
}

