// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// !I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0.clone());// one way. But is not satisfying, as that's not the intent of the println code
    // let mut vec1 = fill_vec(&vec0); //this is the 2nd way according to hint
    let mut vec1 = fill_vec(&mut vec0); //this and all other changes is another way. This is needs more changes but gets the println code to do what it was meant to

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // let mut vec = vec.clone(); // this is another way. way #2 from the hint infact
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
