use std::collections::HashMap;

fn main() {
    // The HashMap can map any hashable type to any other
    // The first type is called the "key"
    // and the second one the "value"
    let mut tv_ratings = HashMap::new();
    tv_ratings.insert("The IT Crowd", 8);
    tv_ratings.insert("13 Reasons Why", 7);
    tv_ratings.insert("House of Cards", 9);
    tv_ratings.insert("Stranger Things", 8);
    tv_ratings.insert("Breaking Bad", 10);

    // Does a key exist?
    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("Did we rate House of Cards? {}", contains_tv_show);
    let contains_tv_show = tv_ratings.contains_key("House");
    println!("Did we rate House? {}", contains_tv_show);

    // Access a value
    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {} out of 10", rating);
    }

    // If we insert a value twice, we overwrite it
    let old_rating = tv_ratings.insert("13 Reasons Why", 9);
    if let Some(old_rating) = old_rating {
        println!("13 Reasons Why's old rating was {} out of 10", old_rating);
    }
    if let Some(rating) = tv_ratings.get("13 Reasons Why") {
        println!("But I changed my mind, it's new rating is {} out of 10", rating);
    }

    // Remove a key and its value
    let removed_value = tv_ratings.remove("The IT Crowd");
    if let Some(removed_value) = removed_value {
        println!("The removed series had a rating of {}", removed_value);
    }

    // Iterating accesses all keys and values
    println!("All ratings:");
    for (key, value) in &tv_ratings {
        println!("{}\t: {}", key, value);
    }

    // We can iterate mutably
    println!("All ratings with 100 as a maximum:");
    for (key, value) in &mut tv_ratings {
        *value *= 10;
        println!("{}\t: {}", key, value);
    }

    // Iterating without referencing the HashMap moves its contents
    for _ in tv_ratings {}
    // tv_ratings is not usable anymore

    // If you don't need to access both keys and values at the same time,
    // you can iterate over either individually

    // Like with the other collections, you can preallocate a size
    // to gain some performance
    let mut ages = HashMap::with_capacity(10);
    ages.insert("Dory", 8);
    ages.insert("Nemo", 5);
    ages.insert("Merlin", 10);
    ages.insert("Bruce", 9);

    // Iterate over all keys
    println!("All names:");
    for name in ages.keys() {
        println!("{}", name);
    }

    // Iterate over all values
    println!("All ages:");
    for age in ages.values() {
        println!("{}", age);
    }

    // Iterate over all values and mutate them
    println!("All ages in 10 years");
    for age in ages.values_mut() {
        *age += 10;
        println!("{}", age);
    }

    // You can use the entry API to assign default values to keys
    // if they're not yet in the HashMap
    {
        let age_of_coral = ages.entry("coral").or_insert(11);
        println!("age_of_coral: {}", age_of_coral);
    }
    let age_of_coral = ages.entry("coral").or_insert(15);
    println!("age_of_coral: {}", age_of_coral);
}