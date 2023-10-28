

pub struct Message<T> {
    receiver: String,
    content: T,
    //callback :fn()
}

pub trait Module {
    //fn is_exclusive()->bool;

    //fn get_messages()->Box<dyn msg_bulletin>;
    fn new(context: winit::event_loop::EventLoop<()>)->Self;

    fn start(&self);
  
    //fn before_frame();

    fn process_frame(&self);

    //fn after_frame();

    fn finish(&self);
}
