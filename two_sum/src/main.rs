use std::collections::HashMap;

// fn main() {
//     let array: [i32; 5] = [0; 5];
//     let target: i32 = 5;
//     two_sum(&array, target)
// }

// fn two_sum(array: &[i32], target: i32) {
//     let mut map: HashMap<i32, i32> = HashMap::new();
//     for x in 0..array.len() {
//         map.insert(x.try_into().unwrap(), array[x]);
//     }
//     for x in 0..array.len() {
//         let difference = target - array[x];
//         if map.contains_key(&difference) && map.get(&x as i32) != array[&x as i32] {
//             break;
//         }
//     }
// }
fn main() {
    let nums = vec![1,2,3,4,5];
    let target: i32 = 5;
    let _result = two_sum(&nums, target);
    println!("{:?}", nums)
}

fn two_sum (nums: &Vec<i32>, target:i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        map.insert(*num, i as i32);
    }
    for (i, num) in nums.iter().enumerate() {
        let difference = target - num;

        if let Some(&index) = map.get(&difference) {
            if index != i as i32 {
                return vec![i as i32, index];
            }
        }
    }

    vec![]
}
