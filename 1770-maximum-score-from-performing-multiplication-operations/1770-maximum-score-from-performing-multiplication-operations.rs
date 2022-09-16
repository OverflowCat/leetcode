use std::collections::HashMap;
impl Solution {
    pub fn maximum_score(数: Vec<i32>, 乘: Vec<i32>) -> i32 {
        type 参 = (usize, usize, usize);
        let mut 记忆: HashMap<参, i32> = HashMap::new();
        fn f(
            左: usize, 右: usize, i: usize,
            数: &Vec<i32>, 乘: &Vec<i32>,
            记忆: &mut HashMap<参, i32>,
        ) -> i32 {
            if i >= 乘.len() { return 0; }
            let k = (左, 右, i);
            if let Some(&x) = 记忆.get(&k) { return x; }
            let 乘数 = 乘[i];
            let i = i + 1;
            let res = (数[左] * 乘数 + f(左 + 1, 右, i, 数, 乘, 记忆))
                  .max(数[右] * 乘数 + f(左, 右 - 1, i, 数, 乘, 记忆));
            记忆.insert(k, res);
            res
        }
        f(0, 数.len() - 1, 0, &数, &乘, &mut 记忆)
    }
}
