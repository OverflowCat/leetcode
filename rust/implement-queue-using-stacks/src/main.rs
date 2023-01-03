use std::default::Default;

struct Queue<T: Default + Clone> {
    s1: Vec<T>,
    s2: Vec<T>,
}

impl<T: Default + Clone> Queue<T> {
    fn new() -> Self {
        Self {
            s1: vec![],
            s2: vec![],
        }
    }

    fn push(&mut self, x: T) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> T {
        self.peek();
        self.s2.pop().unwrap()
    }

    fn peek(&mut self) -> T {
        if self.s2.is_empty() {
            while !self.s1.is_empty() {
                self.s2.push(self.s1.pop().unwrap());
            }
        }
        self.s2.last().unwrap().clone()
    }

    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

type MyQueue = Queue<i32>;
