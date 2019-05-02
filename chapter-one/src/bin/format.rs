fn main() {
    let color = "red";
    // The '{}' in the formatted string gets replaced by the parameter
    let favorite = format!("My favorite color is {}", color);
    println!("{}", favorite);

    // You can add multiple parameters, which will be put in place one after another
    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world); // Prints "hello world!"

    // format! can concatenate any data types that implement the 
    // 'Display' trait, such as numbers
    let favorite_num = format!("My favorite number is {}", 42);
    println!("{}", favorite_num);

    // If you want to include certain parameters multiple times into the string,
    // you can use positional parameters
    let duck_duck_goose = format!("{0}, {0}, {0}, {1}", "duck", "goose");
    println!("{}", duck_duck_goose);

    // You can even name parameters!
    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname="Bond",
        forename="James"
    );
    println!("{}", introduction);
}