use rand::{seq::SliceRandom, Rng};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

fn main() {
    let mut input: Vec<i32> = Vec::new();

    let args = std::env::args().nth(1).unwrap();

    for e in args.split("") {
        match e.parse::<i32>() {
            Ok(iel) => input.push(iel),
            Err(err) => {}
        }
    }

    let mut global: Vec<Vec<i32>> = Vec::new();

    while global.len() < 6 {
        let combination = generate_combination(&input);
        if !global.contains(&combination) {
            global.push(combination);
        }
    }

    println!("global: {:?}", global);
}
