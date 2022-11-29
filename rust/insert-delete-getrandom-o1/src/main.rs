use rand::{Rng, rngs::ThreadRng};
type Set = std::collections::BTreeSet<i32>;
#[derive(Default)]
struct RandomizedSet {
    data: Set,
    rng: ThreadRng
}

impl RandomizedSet {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Default::default()
    }

    #[inline]
    fn insert(&mut self, val: i32) -> bool {
        self.data.insert(val)
    }
    #[inline]
    fn remove(&self, val: i32) -> bool {
        self.data.remove(&val)
    }
    fn get_random(&self) -> i32 {
        
    }
}

/*
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

struct Solution {}
