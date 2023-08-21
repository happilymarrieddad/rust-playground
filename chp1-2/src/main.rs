fn basic_fn() {
    println!("This is a basic function")
}

fn with_inputs(name: &str, salary: i32) {
    println!("The name is {} and the salary is {}", name, salary);
}
fn main() {
    basic_fn();
    with_inputs("Nick", 40_000);

    let full_name = "Nick";
    let salary = 50_000;

    with_inputs(full_name, salary);

    println!("Multiplied value: {}", with_inputs_and_outputs(5, 10));

    println!(
        "Multiple outputs: {:?}",
        with_inputs_and_multiple_outputs(3, 7)
    );

    let (mul, add, sub) = with_inputs_and_multiple_outputs(2, 6);
    println!("Mul: {} Add: {} Sub: {}", mul, add, sub);

    let result = with_inputs_and_multiple_outputs(2, 6);
    println!("Mul: {} Add: {} Sub: {}", result.0, result.1, result.2);

    // Code blocks - have a body but are not named (used once)
    // isolate small bits of code
    let full_name = {
        let first_name = "Nick";
        let last_name = "Kotenberg";

        format!("{} {}", first_name, last_name)
    };

    println!("Full name: {}", full_name);

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("invalid input");

    println!("Received: {n}");
}

// This can be defined before main
fn with_inputs_and_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn with_inputs_and_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
