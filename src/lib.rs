

pub struct Message<T> {
    receiver: String,
    content: T,
    //callback :fn()
}

pub trait Module {
    //fn is_exclusive()->bool;

    //fn get_messages()->Box<dyn msg_bulletin>;

    fn start();
  
    //fn before_frame();

    fn process_frame();

    //fn after_frame();

    fn finish();
}
