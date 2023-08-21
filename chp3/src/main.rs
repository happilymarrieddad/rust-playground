fn main() {
    // One mutable reference in scope
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &mut heap_num;
    let ref2 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Many immutable references
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Mutable and immutable references can not coexists
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    let ref3 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);

    // after scope you can create a mut reference after used
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3 = &mut heap_num;
    println!("ref3: {:?}", ref3);

    // Data should not change when ref is in scope
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    heap_num.push(68); // can not borrow when immutable references to the data
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // this is okay because ref1/2 is out of scope now
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    heap_num.push(68);
}
