use rand::{thread_rng, Rng};
use std::collections::HashMap;
struct VecSet<T> {
    map: HashMap<T, usize>,
    vec: Vec<T>,
}
impl<T> VecSet<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }
    #[inline]
    fn insert(&mut self, elem: T) -> bool {
        if self.map.contains_key(&elem) {
            false
        } else {
            self.vec.push(elem.clone());
            self.map.insert(elem, self.vec.len() - 1);
            true
        }
    }
    fn get_random(&mut self) -> T {
        let index = thread_rng().gen_range(0, self.vec.len());
        self.vec[index].clone()
    }
    fn remove(&mut self, elem: T) -> bool {
        if let Some(index) = self.map.remove(&elem) {
            if index == self.vec.len() - 1 {
                self.vec.pop();
            } else {
                self.map.insert(self.vec.last().unwrap().clone(), index);
                self.vec.swap_remove(index);
            }
            true
        } else {
            false
        }
    }
}
type RandomizedSet = VecSet<i32>;
