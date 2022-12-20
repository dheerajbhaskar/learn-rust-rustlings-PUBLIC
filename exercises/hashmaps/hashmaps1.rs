// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.

// !I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    //dheeraj: better hashmap initialization with a literal: https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
    // let mut basket = HashMap::new();// TODO: declare your hash map here.
    // let mut basket = HashMap::from([
    //     ("pineapple".to_string(), 1 )
    // ]);
    let mut basket: HashMap<_,_> = vec![("jackfruit".to_string(), 3)]
                                   .into_iter()
                                   .collect();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert("apple".to_string(), 1);
    basket.insert("king coconut".to_string(),2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
