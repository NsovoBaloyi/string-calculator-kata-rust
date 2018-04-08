pub fn add(numbers:&str) -> i32 {
    if numbers.is_empty(){
        0
    }
    else{
        numbers.trim().parse::<i32>().unwrap()
    }
}