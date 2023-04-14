use std::{error::Error, process::exit};

use rand::seq::SliceRandom;

// math
fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

// math

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

fn count_unique(input: &Vec<i32>) -> usize {
    let mut tmpd: Vec<i32> = Vec::new();

    for el in input {
        if !tmpd.contains(el) {
            tmpd.push(*el);
        }
    }
    tmpd.len()
}

fn count_possible_combinations(n: usize, m: usize) -> Option<usize> {
    if m == n {
        Some(factorial(n))
    } else if m == 1 {
        Some(1)
    } else {
        let mut i = 1;
        let mut del = 1;
        let mut oldn = factorial(n);

        while i < m {
            oldn = oldn / del;
            del = del + 1;
            i = i + 1;
        }
        Some(oldn)
    }
}

fn main() {
    let mut input: Vec<i32> = Vec::new();

    let args = match std::env::args().nth(1) {
        Some(args) => args,
        None => {
            "123".to_string()
            // eprintln!("no argument provided");
            // exit(1);
        }
    };
    for e in args.split("") {
        match e.parse::<i32>() {
            Ok(iel) => input.push(iel),
            Err(_) => {}
        }
    }

    // let mut global: Vec<Vec<i32>> = Vec::new();
    // while global.len() < 6 {
    //     let combination = generate_combination(&input);
    //     if !global.contains(&combination) {
    //         global.push(combination);
    //     }
    // }
    // println!("global: {:?}", global);
    println!("{:?}", count_possible_combinations(5, 4));
}
