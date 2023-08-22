/* Closures */

fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
    if f(y) == true {
        println!("Division result is {}", x / y);
    } else {
        println!("Division not possible");
    }
}

fn main() {
    let x = 5;

    let square = |num: i32| println!("The square of the variable is {}", num * num);
    square(x);

    let square = |num: i32| println!("The square of the variable is {}", num * num * num);
    let y = 8;
    square(y);

    let print_info = |general_info: String, name: &str, age: i32| {
        println!("{}\n\t {}: {}", general_info, name, age)
    };

    let general_info = String::from("the details are ");
    let (person_name, person_age) = (String::from("Nick"), 39);

    print_info(general_info, &person_name, person_age);

    // the types are determined by the caller and if you try to call with a different type
    // the compiler will complain
    let square = |num| num * num;
    let x = 5;
    square(5);

    let division_status = |y: f32| {
        if y != 0.0 {
            true
        } else {
            false
        }
    };

    division(5.0, 10.0, division_status);
    division(54.0, 0.0, division_status);

    // Closures 2
    let c1 = |x: u32| -> u32 { x + 1 };
    let c2 = |x| x + 1;

    println!("{}", c1(5));
    println!("{}", c2(6));

    let mut v_1: Vec<i32> = vec![1, 2, 3];
    // Captures the scope so even after it still can use it
    let some_closure = || {
        println!("Vec 1: {:?}", v_1);
    };

    println!("Vec 1: {:?}", v_1);
    v_1[1] = 15; // can mutate after the closure has been defined and doesn't affect ownership
    some_closure();
    v_1[2] = 20; // can also update here because the closure call is completed
}
