struct FrontMiddleBackQueue {
    vec: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        return FrontMiddleBackQueue {
            vec: Vec::new()
        }
    }
    
    fn push_front(&mut self, val: i32) {
        self.vec.insert(0, val);
    }
    
    fn push_middle(&mut self, val: i32) {
        let middle_position = self.vec.len() / 2;
        self.vec.insert(middle_position, val);
    }
    
    fn push_back(&mut self, val: i32) {
        self.vec.push(val);
    }
    
    fn pop_front(&mut self) -> i32 {
        if self.vec.len() > 0 {
            let return_val = self.vec[0];
            self.vec.remove(0);
            return return_val;
        } else {
            return -1;
        }
    }
    
    fn pop_middle(&mut self) -> i32 {
        if self.vec.len() > 0 {
            let middle_position = (self.vec.len() + 1) / 2;
            let return_val = self.vec[middle_position - 1];
            self.vec.remove(middle_position - 1);
            return return_val;
        } else {
            return -1;
        }
    }
    
    fn pop_back(&mut self) -> i32 {
        return self.vec.pop().unwrap_or(-1);
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
