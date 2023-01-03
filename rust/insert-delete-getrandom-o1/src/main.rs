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

fn main() {
{
    let mut randomizedSet = RandomizedSet::new();
    randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
    randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
    randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
    randomizedSet.get_random(); // getRandom() should return either 1 or 2 randomly.
    randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
    randomizedSet.insert(2); // 2 was already in the set, so return false.
    assert_eq!(randomizedSet.get_random(), 2); // Since 2 is the only number in the set, getRandom() will always return 2.
}{
    let mut randomizedSet = RandomizedSet::new();
    randomizedSet.insert(0); // 0
    randomizedSet.insert(1); // 0,1
    randomizedSet.remove(0); // 1
    assert_eq!(1,randomizedSet.get_random()); // 1
    println!("{:?}, {:?}", randomizedSet.vec, randomizedSet.map);
    randomizedSet.insert(2); // 2 was already in the set, so return false.
    println!("{:?}, {:?}", randomizedSet.vec, randomizedSet.map);
    randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
    println!("{:?}, {:?}", randomizedSet.vec, randomizedSet.map);
    assert_eq!(randomizedSet.get_random(), 2); // Since 2 is the only number in the set, getRandom() will always return 2.
}}
