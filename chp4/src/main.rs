fn main() {
    // Conditionals

    let num = 40;
    if num < 50 {
        println!("The number is less than 50");
    }

    println!("Line run");

    let marks = 65;
    if marks >= 60 && (marks <= 70 || marks != 0) {
        println!("marks are inbetween");
    }

    println!("Enter a number");
    let mut some_num = String::new();

    std::io::stdin()
        .read_line(&mut some_num)
        .expect("failed to read input");

    let some_num: i32 = some_num.trim().parse().expect("invalid input");
    if some_num != 0 {
        if some_num % 2 == 0 {
            println!("even");
        } else {
            println!("odd");
        }
    } else {
        println!("numer is either even or odd");
    }

    // if let
    let n = if true { 5 } else { 4 };
    println!("n is {}", n);

    // Match - control flow operator
    /*
     * match value {
     *       possible_value(s) => {statements to execute},
     *       possible_value(s) => {statements to execute},
     *       possible_value(s) => {statements to execute},
     *       _ => {default execution of some statements}
     * }
     */
    let some_number = 100;

    match some_number {
        1 => println!("the number is 1"),
        2 | 3 => println!("the number is 2 or 3"),
        4..=100 => println!("number is 4 up to 100 including"),
        _ => println!("default found"),
    }

    let marks = 50;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => {
            println!("B grade found");
            'B'
        }
        _ => 'F',
    };

    println!("Grade is {}", grade);

    // Loops

    /*
     * Infinite loop
     * loop {
     * }
     */
    let num = 5;
    let mut guess = false;

    println!("Guess my number which is inbetween 1 and 20");

    while guess != true {
        let mut guessed_number = String::new();
        std::io::stdin()
            .read_line(&mut guessed_number)
            .expect("invalid input");

        let number: i32 = guessed_number.trim().parse().expect("invalid input");

        guess = number == num;
    }

    // for
    let vect = vec![4, 6, 5, 6, 9, 20];
    for i in 0..=5 {
        println!("{} in {}", i, vect[i]);
    }

    println!("Does vec have a 5: {}", hasValue(5, vect.clone()));
    println!("Does vec have a 5: {}", hasValue(99, vect));

    let mut mp = vec![1, 2, 3];
    for n in mp.iter() {
        // doesn't change ownership
        println!("{}", n);
    }

    for n in &mp {
        // doesn't change ownership
        println!("{}", n);
    }

    for n in mp.iter_mut() {
        // doesn't change ownership
        *n += 1;
        println!("{}", n);
    }

    println!("{:?}", mp);
}

// Bad function but it illustrates that a loop can return a value
fn hasValue(value_to_look_for: i32, v: Vec<i32>) -> bool {
    let mut i = 0;
    let res = loop {
        if i >= v.len() {
            break false;
        }
        if v[i] == value_to_look_for {
            break true;
        }
        i += 1;
    };

    res
}
