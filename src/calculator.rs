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
    let re = Regex::new(r"[,|\n]+").unwrap();

    re.split(numbers).map(|i| i.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn sum_list(list:Vec<i32>) -> i32 {
    list.into_iter().sum()
}