#![allow(unused_mut)]
///History Buffer
///Struct used to store and manipulate input commands
pub struct HistoryBuffer {
    histvec: Vec<String>,
}

impl HistoryBuffer {
    ///Instantiate HistoryBuffer with an empty vector
    ///to store input lines
    pub fn new() -> Self {
        let mut vec: Vec<String> = Vec::new();
        HistoryBuffer {
            histvec: vec,
        }
    }

    ///Pushes a new value into the History Stack
    pub fn store(&mut self, input: String) {
        self.histvec.push(input);
    }

    pub fn pop(&mut self) -> String {
        let popped_option = self.histvec.pop();
        if !popped_option.is_some() {
            "".to_owned()
        } else {
            let popped = popped_option.expect("Popped a None value");
            self.histvec.insert(0, popped.clone());
            popped
        }
    }
}
