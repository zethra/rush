//This is used to create a queue of commands so that interpret can
//run the correct order of things that need to be done and which ones
//are in parallel. The Queue keeps a record of how many commands need to be
//run in parallel and keeps track of them with multiple vectors

#[derive(Debug)]
pub struct Opqueue {
    pub commands: Vec<Vec<Operation>>,
    pub parallel: u8,
}

//Enum to determine which operation should be done
#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Parallel,
    And,
    Xor,
    Or,
    Nand,
    Not,
    Pipe,
}

impl Opqueue {
    pub fn new() -> Self {
        Opqueue {
            commands: Vec::new(),
            parallel: 0,
        }
    }

    //Use this to push new queues onto the stack
    pub fn push(&mut self, par_vec: Vec<Operation>) {
       self.commands.push(par_vec);
       self.parallel += 1;
    }

    //Used to pop off queues from the stack if they're available
    pub fn pop(&mut self) -> Option<Vec<Operation>> {
        if self.parallel > 0 {
            self.parallel -= 1;
        }
        self.commands.pop()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opqueue_new() {
        //Initialize new Opqueue and check for defaults
        let queue = Opqueue::new();
        assert_eq!(queue.parallel, 0);
        assert_eq!(queue.commands.len(),0);
    }

    #[test]
    fn opqueue_push_pop() {
        //Initialize New Opqueue
        let mut queue = Opqueue::new();
        assert_eq!(queue.parallel, 0);
        assert_eq!(queue.commands.len(),0);

        //Set up two Queues of Ops
        let parallel1: Vec<Operation> = Vec::new();
        let parallel2: Vec<Operation> = Vec::new();

        //Push first and test for new values
        queue.push(parallel1);
        assert_eq!(queue.parallel, 1);
        assert_eq!(queue.commands.len(),1);

        //Push second and test for new values
        queue.push(parallel2);
        assert_eq!(queue.parallel, 2);
        assert_eq!(queue.commands.len(),2);

        //Pop first value
        assert_eq!(queue.pop(),Some);
        assert_eq!(queue.parallel, 1);
        assert_eq!(queue.commands.len(),1);

        //Pop second value
        assert_eq!(queue.pop(),Some);
        assert_eq!(queue.parallel, 0);
        assert_eq!(queue.commands.len(),0);

        //Check for exhaustion
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.parallel, 0);
        assert_eq!(queue.commands.len(),0);

    }

}
