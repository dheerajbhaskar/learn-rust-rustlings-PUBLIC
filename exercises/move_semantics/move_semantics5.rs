// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// !I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; //dheeraj: y is dropped here because it's not referenced after this
    let z = &mut x;
    *z += 1000;
    // dbg!(y); //can't do this. If y referenced here, then it needs to be alive till here. That would mean two mutable references would be alive at the same time, which is not allowed.
    assert_eq!(x, 1200);
}
