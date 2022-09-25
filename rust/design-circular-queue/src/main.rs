struct MyCircularQueue {
    dat: Vec<i32>,
    siz: usize,
    ini: usize,
    end: usize,
    len: usize,
}

impl MyCircularQueue {
    #[inline]
    fn overflow(&self, i: usize) -> usize {
        if i == self.siz {
            0
        } else {
            i
        }
    }

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            dat: vec![0; k],
            siz: k,
            ini: 0,
            end: 0,
            len: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.siz == self.len {
            false
        } else {
            self.dat[self.end] = value;
            self.end = self.overflow(self.end + 1);
            self.len += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.len == 0 {
            false
        } else {
            self.ini = self.overflow(self.ini + 1);
            self.len -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.len == 0 {
            -1
        } else {
            self.dat[self.ini]
        }
    }

    fn rear(&self) -> i32 {
        if self.len == 0 {
            -1
        } else {
            self.dat[if self.end == 0 {
                self.siz - 1
            } else {
                self.end - 1
            }]
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.siz
    }
}
