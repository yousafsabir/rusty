fn main() {
    let num: i32 = 127;
    let result = power_of_two(num);
    if result {
        println!("{}", "Given Number is power of two")
    } else {
        println!("{}", "Given Number is not power of two")
    }
}

fn power_of_two(num: i32) -> bool {
    if (num & (num - 1)) != 0 {
        return false
    } else {
        return true
    }
}
