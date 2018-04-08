use regex::Regex;

pub fn add(numbers:&str) -> i32 {
    let re = Regex::new(r"[,]+").unwrap();

    if numbers.is_empty(){
        0
    }
    else{
         re.split(numbers).map(|i| i.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>().into_iter().sum()
    }
}