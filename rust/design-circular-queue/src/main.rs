struct MyCircularQueue {
    data: Vec<i32>,
    k: usize,
    首: usize,
    尾: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    #[inline]
    fn overflow(&self, i: usize) -> usize {
        if i == self.k {
            0
        } else {
            i
        }
    }

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            data: vec![0; k],
            k,
            首: 0,
            尾: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        let next_尾 = self.overflow(self.尾 + 1);
        if next_尾 == self.首 {
            false
        } else {
            self.data[self.尾] = value;
            self.尾 = next_尾;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.首 == self.尾 {
            false
        } else {
            self.首 = self.overflow(self.首 + 1);
            true
        }
    }

    fn front(&self) -> i32 {
        self.data[self.首]
    }

    fn rear(&self) -> i32 {
        self.data[if self.尾 == 0 {self.k - 1} else {self.尾}]
    }

    fn is_empty(&self) -> bool {
        self.首 == self.尾
    }

    fn is_full(&self) -> bool {
        self.首 + 1 == self.尾
    }
}

/*
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
