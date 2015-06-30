// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both outlive
// the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);

    // Suppose this `drop` worked. Then `print_refs` would outlive
    // `x` and `four` from `main()` would refer to erased data.
    // These two things go hand in hand and so both are banned.
    //
    //drop(*x);
    // ERROR: cannot move out of borrowed content
}

// `failed_borrow` takes no references and returns nothing but has
// a single lifetime `'a` which must outlive the function.
fn failed_borrow<'a>() {
    let _x = 12;

    // Attempting to use the lifetime `'a` as an explicit type
    // annotation inside the function will fail because the
    // lifetime `'a` doesn't match the lifetime that `y` has.
    // `y` starts inside `failed_borrow` and so it is smaller.
    //
    //let y: &'a i32 = &_x;
    // ERROR: `_x` does not live long enough
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}
