use std::process::exit;

use rand::Rng;

// math
fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

// math
fn count_unique(input: &Vec<usize>) -> usize {
    let mut tmpd: Vec<usize> = Vec::new();

    for el in input {
        if !tmpd.contains(el) {
            tmpd.push(*el);
        }
    }
    tmpd.len()
}
fn multiply_unique(n: usize, m: usize) -> usize {
    let mut res = 1;
    let mut i = 1;
    while i < (n - m) + 1 {
        res = res * i;
        i = i + 1;
    }
    res
}
fn count_possible_combinations(n: usize, m: usize) -> usize {
    return factorial(n) / multiply_unique(n, m);
}

fn join_usize(a: &Vec<usize>) -> usize {
    let mut t: Vec<String> = Vec::new();
    for el in a {
        t.push(el.to_string());
    }
    let l = t.join("");

    return l.parse::<usize>().unwrap();
}

fn main() {
    let mut input: Vec<usize> = Vec::new();

    let args = match std::env::args().nth(1) {
        Some(args) => args,
        None => {
            eprintln!("no argument provided");
            exit(1);
        }
    };

    for e in args.split("") {
        match e.parse::<usize>() {
            Ok(iel) => input.push(iel),
            Err(_) => {}
        }
    }

    let count_chars = input.len();
    let count_unique_of_input = count_unique(&input);
    let multiply = multiply_unique(count_chars, count_unique_of_input);
    let combo_count = factorial(count_chars) / multiply;
    // main

    // output
    let mut global: Vec<usize> = Vec::new();

    global.push(join_usize(&input));
    while global.len() < combo_count {
        let mut num: Vec<usize> = Vec::new();
        num.resize(count_chars, 0);
        let mut input1 = input.clone();
        let mut i = 0;
        let mut ran = 0;
        while i < num.len() {
            if input1.len() != 0 {
                ran = rand::thread_rng().gen_range(0..input1.len());
            } else {
                ran = 0;
            }
            num[i] = input1[ran];

            input1.remove(ran);

            i = i + 1;
        }
        if !global.contains(&join_usize(&num)) {
            global.push(join_usize(&num));
        }
    }
    println!("global: {:?}", global);

    // main
}
