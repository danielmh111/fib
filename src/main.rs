use std::io;

fn main() {
    println!("what is the index of the fibonacci number you would like?");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("could not read line");

    let n: i128 = input    
        .trim()
        .parse()
        .expect("could not interpret input. Please enter an integer.");

    // let output: i128 = get_nth_fibonacci(n);
    let output: i128 = get_nth_recursively(n);
    println!("{}", output)
}


fn get_nth_fibonacci(n: i128) -> i128 {
    let mut a: i128 = 1;
    let mut b: i128 = 1;

    let mut counter: i128 = 1;

    while counter <= n {
        let holder: i128 = b;
        b = a + b;
        a = holder;
        counter += 1;
    }

    return a 
}


fn get_nth_recursively(n: i128) -> i128 {
    match n {
        1 => return 1,
        2 => return 1,
        _ => return get_nth_recursively(n-2) + get_nth_recursively(n-1)
    }
}
