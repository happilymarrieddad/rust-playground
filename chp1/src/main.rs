fn main() {
    // Variables
    println!("Hello, world!");

    println!("{}", 10);

    println!("Some value {language}", language = 15);

    println!("Max value i64: {}", std::i64::MAX);

    println!("some values {:?}", (34, "TEST"));

    let x2;

    x2 = 5;

    println!("{}", x2);

    let x = 255;

    println!("Octal {:o} Hex: {:X} Binary: {:b}", x, x, x);

    let n1: i32 = 10;
    let n2: f64 = 15.5;

    println!("Convert {}", n1 as f64 + n2);

    // Strings
    let name: String = "Nick Kotenberg".to_string();
    let some_string: &str = "Fixed length string";
    let mut growable_string: String = String::from("some value");
    println!("The string is \"{}\"", growable_string);

    growable_string.push('s');
    println!("The string is \"{}\"", growable_string);

    growable_string.pop();
    println!("The string is \"{}\"", growable_string);

    growable_string.push_str(" which I will use");
    println!("The string is \"{}\"", growable_string);

    println!(
        "Basic functions on strings,
        is_empty(): {},
        length(): {},
        Bytes(): {},
        Contains use: {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use"),
    );

    growable_string.push_str("    ");
    let str_len = growable_string.trim().len();

    let number = 6;
    let num_str = number.to_string();

    println!("Is the number a string: {}", num_str == "6");

    let some_char = 'a';
    let char_str = some_char.to_string();

    let my_name = "Nick".to_string();

    let empty_str = String::new();
    println!("Length of str: {}", empty_str.len());

    let s_1 = "Nick".to_string();
    let s_2 = "Kotenberg".to_string();

    let s_3 = format!("My first name is {} and my last name is {}", s_1, s_2);
    println!("Val: {}", s_3);

    println!("Concat: {}", format!("{} {}", s_1, s_2));

    // Tuples and Arrays
    let my_information = ("Salary", 40_000);
    println!("{} is equal to {}", my_information.0, my_information.1);

    println!("Printing the whole tuple: {:?}", my_information);

    let (salary, salary_value) = my_information;
    println!("First: {} Second: {}", salary, salary_value);

    let salary = my_information.0;
    let salary_value = my_information.1;

    let nested_tuple = (4, 5.0, (3, 2), my_information);

    let element = nested_tuple.2 .0;
    println!("Element: {}", element);

    let empty_tuple = ();

    let mut number_array = [4, 5, 6, 8, 9];
    println!("{}", number_array[0]);
    println!("{:?}", number_array);

    number_array[4] = 5;

    let array_with_same_elements = [0; 10];
    let mut string_array_1 = ["apple", "tomato", "grapes"];
    let string_array_2 = ["unknown"; 6];
    string_array_1[0] = "kamran azam";

    let char_array = ['a', 'p', 'p', 'l', 'e'];

    let mut number_array_1 = [4, 5, 6, 8, 9];

    // First 3 values of the above array - do not include the last index in the array (3)
    let subset_array = &number_array_1[0..3];
    println!("Subset of array: {:?}", subset_array);

    // Force inclusion of last value of subset - include the last index in the array (=3)
    let subset_array_2 = &number_array_1[0..=3];
    println!("Subset of array 2: {:?}", subset_array_2);

    println!("Len of array: {}", subset_array_2.len());
    println!(
        "Array occupies {} bytes",
        std::mem::size_of_val(&number_array_1)
    );
}
