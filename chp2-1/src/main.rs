/**
 * Ownership and References
 */

fn main() {
    // short answer - primitives (can not grow and can not be empty)
    let x: f64 = 32.6;
    let y: f64 = x; // move

    println!("x: {} y: {}", x, y);

    // short answer - not primitive (can grow and be empty)
    let s1: String = String::from("abc");
    let s2: &String = &s1; // move

    println!("s1: {} s2: {}", s1, s2);

    let vec_1 = vec![5, 6, 7, 8, 9];
    let vec_2 = vec_1.clone(); // copy

    println!("Vec 1: {:?} Vec 2: {:?}", vec_1, vec_2);

    // When owner goes out of scope it drops ownership
    let outside_scope_name = {
        let my_name = String::from("Nick Kotenberg");

        my_name // return my name
    };

    // After the Code Block the var my_name is out of scope
    //println!("My name is {}", my_name);
    println!("My name is {}", outside_scope_name);

    // Application Memory (Heap and Stack)
    let (x, y) = (2, 4);

    let sum_value = square_sum(x, y);

    println!("The value of square of sum = {}", sum_value);

    // Strings and Vectors are stored in the Heap (space can grow)
    // Everything else is in the stack (space is pre-assigned)

    let stack_num = 32;
    let mut heap_vec = vec![4, 5, 6];

    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

    // If not cloned, the ownership goes to the function and the value will be dropped after the function ends
    heap_function(heap_vec.clone());
    heap_function_ref(&heap_vec); // here we reference the value instead of copying so no ownership change
    println!("The value inside the main of heap vec: {:?}", heap_vec);

    heap_function_ref_mut(&mut heap_vec);
    println!("The value inside the main of heap vec: {:?}", heap_vec);

    let large_data1 = String::from("this is the first long string");
    let large_data2 = String::from("this is the second long string");

    // This does not copy but uses the references instead - much more efficient
    let huge_data = vec![&large_data1, &large_data2];
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("Var: {}", var)
}

fn heap_function(var: Vec<i32>) {
    println!("Var: {:?}", var)
}

fn heap_function_ref(var: &Vec<i32>) {
    println!("Var: {:?}", var)
}

fn heap_function_ref_mut(var: &mut Vec<i32>) {
    var.push(50);
    println!("Var: {:?}", var)
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let result = square(num1 + num2);

    result
}

fn square(num: i32) -> i32 {
    num * num
}
