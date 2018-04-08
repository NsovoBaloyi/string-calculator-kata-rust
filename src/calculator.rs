use regex::Regex;
use std::vec::Vec;

pub fn add(numbers:&str) -> i32 {

    if numbers.is_empty(){
        0
    }
    else{
         sum_list(parse_numbers(numbers))
    }
}

fn parse_numbers(numbers:&str) -> Vec<i32>{
    let mut del = r"[,|\n]";
    let values;
    
    if numbers.starts_with("//"){
        del = &numbers[2..3];
        values = &numbers[4..];
    }else {
        values = &numbers;
    }

    let re = Regex::new(del).unwrap();

    re.split(values).map(|i| i.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn sum_list(list:Vec<i32>) -> i32 {
    list.into_iter().sum()
}