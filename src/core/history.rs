pub struct History<'b> {
    history: Vec<Vec<&'b str>>,
    line: usize,
    size_cap: usize,
}
//We want this to last the whole time rusty is running so 'static is an appropriate lifetime
//here. Also gets the compiler to shut the hell up
impl History {
    pub fn new<'b>() -> History<'b> {
        //Read from file. If no file create it. Cap history length in config otherwise default on
        //size
        History{
            history: Vec::new(),
            line: 0,
            size_cap: 200,
        }
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
    pub fn push(&mut self, vec: Vec<&str>) {
        self.history.push(vec);
    }
    pub fn get(&self, vec_point: usize) -> &Vec<&str> {
        &self.history.get(vec_point)
    }
    pub fn get_mut(&mut self, vec_point: usize) -> &mut Vec<&str> {
        self.history.get_mut(vec_point)
    }
}
