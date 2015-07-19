pub struct History<'b> {
    pub lines: Vec<Vec<&'b str>>,
    line_number: usize,
    size_cap: usize,
}
//We want this to last the whole time rusty is running so 'static is an appropriate lifetime
//here. Also gets the compiler to shut the hell up
impl <'b> History <'b> {
    pub fn new() -> History<'b> {
        //Read from file. If no file create it. Cap history length in config otherwise default on
        //size
        History{
            lines: Vec::new(),
            line_number: 0,
            size_cap: 200,
        }
    }
    pub fn cap_it(&mut self) {
        //if size of history is above the size cap this function maintains the cap by removing
        //history over time
        loop {
            if self.lines.len() > self.size_cap {
                self.lines.remove(0);
            } else {
                break;
            }
        }
    }
    pub fn push(&mut self, vec: Vec<&'b str>) {
        self.lines.push(vec);
    }
}
