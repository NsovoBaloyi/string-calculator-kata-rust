use regex::Regex;
use std::vec::Vec;
use std::ops::Add;

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
    let mut values;

    if numbers.starts_with("//["){
        del = &numbers[2..numbers.find("\n").unwrap()];
        values = &numbers[numbers.find("\n").unwrap()+1..];
    }
    else if numbers.starts_with("//"){
        del = &numbers[2..3];
        values = &numbers[4..];
    }else {
        values = &numbers;
    }

    let re = Regex::new(&*del.replace("][", "|").add("+")).unwrap();

    re.split(values).map(|i| i.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn sum_list(list:Vec<i32>) -> i32 {
    list.into_iter().sum()
}