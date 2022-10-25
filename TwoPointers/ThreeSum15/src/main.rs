fn main() {
    println!("Hello, world!");
    let res = Solution::three_sum(vec![-1,0,1,2,-1,-4]);
    println!("{:?}", res);

    let res = Solution::three_sum(vec![0, 0, 0]);
    println!("{:?}", res);

    let res = Solution::three_sum(vec![0, 1, 1]);
    println!("{:?}", res);
}

struct Solution{}


//impl Solution {
    //pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        //nums.sort();
        //let mut exists = HashMap::with_capacity(nums.len());
        //for x in nums.iter() {
            //if let Some(count) = exists.insert(*x, 1) {
                //exists.insert(*x, count + 1);
            //}
        //}
        //nums.dedup();
        
        //let mut res = Vec::with_capacity(nums.len() / 3);
        //let mut i = 0;
        //let mut j = nums.len() - 1;
        //while i < j {
            //let comp = nums[j] - nums[i];
            //let check = exists.get(&comp);
            
            //match (i - j).partial_cmp(&0).unwrap() {
                //Ordering::Equal => {
                    
                //},
                //Ordering::Less => {},
                //Ordering::Greater => {},
            //}
        //}

        //res
    //}
//}



//impl Solution {
    //pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        //nums.sort();
        //let mut exists: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        //for x in nums.iter() {
            //if let Some(count) = exists.insert(*x, 0) {
                //exists.insert(*x, count + 1);
            //}
        //}
        //nums.dedup();

        //let mut res: Vec<Vec<i32>> = Vec::new();

        //for (i, x) in nums.iter().enumerate().filter(|&(_, x)| x <= &0) {
            ////exists.insert(*x, *exists.get(x).unwrap() - 1);
            //let upper = - x - nums.last().unwrap();
            //for y in nums[i+1..].iter().rev().filter(|&y| y >= &upper) {
                //if let Some(&count) = exists.get(&(-x-y)) {
                    ////if *x <= -x-y && -x-y <= *y {
                    //res.extend(std::iter::once(vec![*x, -x-y, *y]));
                    ////}
                //}
            //}
        //}

        //res
    //}
//}

// This one was horrible. I tried to get way too clever at first. Confusing!
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();

        for (i, &x) in nums.iter().enumerate() {
            let mut a = i + 1;
            let mut b = nums.len() - 1;
            if 0 < i && nums[i-1] == nums[i] {
                continue
            }

            while a < b {
                match (nums[a] + nums[b]).partial_cmp(&-x).unwrap() {
                    Ordering::Equal => {
                        let next = vec![x, nums[a], nums[b]];
                        while a < b && nums[a] == next[1] { a += 1 }
                        while a < b && nums[b] == next[2] { b -= 1 }
                        res.push(next);
                    },
                    Ordering::Less => a += 1,
                    Ordering::Greater => b -= 1,
                }
            }
        }

        res
    }
}
