use regex::Regex;
use std::vec::Vec;
use std::ops::Add;

pub fn add(numbers:&str) -> i32 {

    if numbers.is_empty(){
        0
    }
    else{
         sum_list(filter_numbers(validate(parse_numbers(numbers))))
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

fn validate(list:Vec<i32>) -> Vec<i32> {
    let negatives = list.clone().into_iter().filter(|i| i < &0 ).collect::<Vec<i32>>();

    if !negatives.is_empty(){
        panic!("Negatives not allowed: {:?}", negatives);
    }

    list
}

fn filter_numbers(list:Vec<i32>) -> Vec<i32> {
    list.into_iter().filter(|i| i < &1001).collect::<Vec<i32>>()
}

fn sum_list(list:Vec<i32>) -> i32 {
    list.into_iter().sum()
}