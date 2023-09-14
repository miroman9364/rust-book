
pub fn play_with_box() {
    let mut b: Box<i32> = Box::new(5);
    println!("b = {b}");

    *b += 1;
    println!("b = {b}");

    let b = Box::new([7; 3]);
    println!("b[0] = {}", b[0]);
}

pub fn ownership_examples()
{
    println!("");
    println!("");
    println!("Ownership examples:");
    println!("");

    // x isn't going to change in this code, so it doesn't need to be mutable
    let x: Box<i32> = Box::new(5);
    println!("x owns the box, x = {x}, the value of x is reached through the pointer to x: {}", *x);

    // a will take ownership of the box and change it, so make a mutable
    let mut a = x;
    println!("a (mutable) now owns the box, a = {a}");

    // box is on the heap, and in this case, it's a pointer to an i32, so to change its value we need to dereference it
    *a += 1;
    println!("a = {a}");

    let r1: &Box<i32> = &a;
    println!("r1 references a, and can act like a for reading the box: r1 = {r1}");

    let mut b: i32 = **r1;
    println!("double dereference of r1 gives us the value from the heap, so b = {b} is a copy of the value in the heap, not a reference to the value in the heap.");

    b += 1;
    println!("changing b to {b} == 7, should not change the value in the heap, so a = {a} should == 6?");
    assert_eq!(b, 7);
    assert_eq!(*a, 6);

    let mut r2 = &mut *a;
    println!("r2 is a reference to the value in the heap, r2 = {r2}, and its value *r2 = {}, and the values should currently be 6", *r2);
    assert_eq!(*r2, 6);

    *r2 += 1;
    print!("changing r2 to {r2} == 7");
    assert_eq!(*r2, 7);

    let mut other_box = Box::new(0);
    r2 = &mut *other_box;
    println!(", should also change the value in the heap, so a = {} == 7?", *a);
    assert_eq!(*a, 7);

    println!("also, r2 is no longer referencing a, but other_box, so r2 = {r2} == 0?");
    println!("the last bit with r2 was necessary in order to stop borrowing a and allow us to use a again. there is probably some better way to do this");
}
