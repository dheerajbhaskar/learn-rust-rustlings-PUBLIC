// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// !I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let mut v = vec![];// TODO: declare your vector here with the macro for vectors
    v.extend(a);//did this because the above TODO wanted a declaration. Also, copying manually would be so menial! ;)
    // let v2:Vec<i32> = vec![].extend(a); //error here that I was breaking my head with was that v2 gets the output of `extend` which is a () or void or unit in rust parlance
    // assert_eq!(v2[..], v[..]);
    // also, I'm not happy with debugging of rust programs
    // it took me a lot of time because. The way rustlings is setup is swallowing all the println!s that I'm putting here
    
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
