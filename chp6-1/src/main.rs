#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let s_1 = "Hello";
    let v: &str;
    {
        let s_2 = String::from("World");
        v = some_fn(s_1, s_2.as_str());
    }

    println!("{}", v);

    let a: i32 = 5;
    {
        let b: i32 = 6;
        println!("{}", greater(&a, &b));
    }
    // This would cause an error because the lifetime for second
    //  is not long enough
    //println!("{}", greater(&a, &b));

    // Struct lifetimes
    let first_name = "Nick";
    let mut nick = Person {
        name: &first_name,
        age: 39,
    };

    {
        let last_name = String::from("John");
        nick.name = &last_name;
    }

    // will error because it doesn't live as long as the struct instance
    //println!("{:?}", nick);

    //
    let some_vec = vec![5, 8, 9, 8, 7, 5, 2];

    // without lifetime reference compiler will complain even though they are the same
    let ret_vec = use_vec(&some_vec, &some_vec);
}

fn use_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}

// a/b are lifetimes and the output needs to know what lifetime should be returned
// lifetimes are only when returning a reference/pointer
fn some_fn<'a, 'b>(f: &'a str, b: &'b str) -> &'a str {
    f
}

// first has a different lifetime than second - the lifetime is required to be
//  as long as the smallest lifetime (I think...)
fn greater<'a, 'b>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        i
    } else {
        j
    }
}
