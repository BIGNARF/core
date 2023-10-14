use std::vec::Vec;

    pub struct OPController {
        processing: Vec<String>,
        scheduled: Vec<String>,
    }
    impl OPController {
        pub fn new(size: u32) -> OPController {
            let mut random_texts: Vec<String> = Vec::new();
            for _ in 0..size {
                let random_string = String::from("TEXT"); // Adjust the range as needed
                random_texts.push(random_string);
            }
            OPController {
                processing: random_texts,
                scheduled: Vec::new(),
            }
        }

        pub fn display(&self){
            for text in &self.processing{
                println!("{}",text);
            }
        }
    }


    pub trait msg_bulletin{

    }

    pub trait Module {
        //fn is_exclusive()->bool;

        //fn get_messages()->Box<dyn msg_bulletin>;

        fn start();

        //fn before_frame();

        //fn after_frame();

        fn finish();
    }