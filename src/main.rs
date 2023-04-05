use std::{collections::HashMap, process::exit};

use rand::seq::SliceRandom;

fn generate_combination(input: &Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    while output.len() < input.len() {
        let i: i32 = *input.choose(&mut rand::thread_rng()).unwrap();
        if !output.contains(&i) {
            output.push(i);
        }
    }
    return output;
}

fn combos_cmp(a: Vec<i32>, b: Vec<i32>) -> bool {
    let cmp = a.iter().zip(&b).filter(|&(a, b)| a == b).count();

    if cmp == b.len() {
        true
    } else {
        false
    }
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

pub fn contains_duplicate(vec: &Vec<usize>) -> bool {
    let mut map = HashMap::<usize, usize>::new();
    for (i, num) in vec.iter().enumerate() {
        match map.contains_key(num) {
            true => return true,
            false => map.insert(*num, i),
        };
    }
    false
}

fn count_possible_combinations(input: &mut Vec<i32>) -> usize {
    let c = input.clone();

    input.dedup();

    let same = (c.len() - input.len()) + 1;

    return factorial(same);
}

fn main() {
    let mut input: Vec<i32> = Vec::new();

    let args = match std::env::args().nth(1) {
        Some(args) => args,
        None => {
            eprintln!("no argument provided");
            exit(1);
        }
    };

    for e in args.split("") {
        match e.parse::<i32>() {
            Ok(iel) => input.push(iel),
            Err(_) => {}
        }
    }

    let mut global: Vec<Vec<i32>> = Vec::new();
    println!("cpc: {}", count_possible_combinations(&mut input));

    while global.len() < 6 {
        let combination = generate_combination(&input);
        if !global.contains(&combination) {
            global.push(combination);
        }
    }
    println!("global: {:?}", global);
}
