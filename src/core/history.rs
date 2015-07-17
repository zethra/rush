pub struct History<'a> {
    history: Vec<Vec<&'a str>>,
    line: usize,
    size_cap: usize,
}
//We want this to last the whole time rusty is running so 'static is an appropriate lifetime
//here. Also gets the compiler to shut the hell up
impl History<'static> {
    pub fn new() -> History<'static> {
        //Read from file. If no file create it. Cap history length in config otherwise default on
        //size
        unimplemented!()
    }
    fn cap_it(&mut self) {
        //if size of history is above the size cap this function maintains the cap by removing
        //history over time
        loop {
            if self.history.len() > self.size_cap {
                self.history.remove(0);
            } else {
                break;
            }
        }
    }
    pub fn push(&mut self, vec: Vec<&'static str>) {
        self.history.push(vec);
    }
    pub fn get(&self, vec_point: usize) {
        self.history.get(vec_point);
    }
    pub fn get_mut(&mut self, vec_point: usize) {
        self.history.get_mut(vec_point);
    }
}
