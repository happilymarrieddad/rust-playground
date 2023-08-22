struct Person {
    name: String,
    age: i32,
}

// Typically match is used with Enums
fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n * n),
        None => None,
    }
}

fn division(divident: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        return Err(String::from("Error: division by zero"));
    }

    Ok(divident / divisor)

    // Using floats in match is no longer acccepted by the compiler
    // match divisor {
    //     0.0 => Err(String::from("Error: division by zero")),
    //     _ => Ok(divident / divisor),
    // }
}

fn main() {
    let mut disease: Option<String> = None;

    disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease_name) => println!("You have the disease of {}", disease_name),
        None => println!("You have no disease"),
    }

    let s1 = Some("Some String");
    println!(
        "The value of s1 is {:?} and the value of s1 itself after unwrapping is {:?}",
        s1,
        s1.unwrap()
    );

    let f1 = Some(10.65);
    let mut f2 = 16.5;

    f2 = f2 + f1.unwrap();
    println!("F2: {}", f2);

    let v1 = Some(vec![0, 1, 2, 3]);

    let p1 = Person {
        name: String::from("Nick Kotenberg"),
        age: 39,
    };

    let someone = Some(p1);

    let num = Some(6);
    if square(num) != None {
        println!("The result of the square is {:?}", square(Some(6)).unwrap());
    } else {
        println!("We do not have any value");
    }

    println!("{:?}", square(None));

    // Results
    println!("{:?}", division(9.0, 4.5));
    println!("{:?}", division(0.0, 4.5));
    println!("{:?}", division(1.0, 0.));

    // vector results
    let some_vec = vec![5, 5, 2, 1, 5, 9];
    let result = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err(String::from("no value found")),
    };

    println!("The result is {:?}", result);
}
