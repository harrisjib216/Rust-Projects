fn main() {
    ownership_intro();

    variables_and_data();

    // passing ownership example
    let string = String::from("data");
    takes_ownership(string);
    // the line below is going to fail
    // println!("{}", string);

    // making a copy example
    let number = 23;
    makes_copy(number);
    println!("{}", number);

    // lets try passing around data
    let new_string: String = give_ownership();
    let new_string: String = take_and_give_ownership(new_string);
    let new_string: String = take_modify_return_ownership(new_string);
    println!("End result: {}", new_string);

    let mut string2 = String::from("RUST LANGUAGE");
    modify_with_referrence(&mut string2);
    println!("End result: {}", string2);

    referrences();

    referrence_constraints();
}

fn ownership_intro() {
    /*
        ownership rules
    1. each VARIABLE in rust is an OWNER to a VALUE
    2. there can only be ONE ONWER at a time
    3. when the OWNER leaves/goes out of scope, the VALUE is DROPPED
    */

    // variable scope example
    // here we create a new scope and a variable inside of it
    {
        // at the moment, "test" has not been defined, so it does not exist
        let test: &str = "created in memory!"; // now OWNER and VALUE have been created

        // now we are able to do things with this owner
        println!("{}", test);
    } // since we have left the scope, the owner and value have been dropped
}

fn variables_and_data() {
    // lets try with some stack/scalar data
    let x = 5;
    let y = x; // the value 5 is copied into x
    println!("x: {}, y: {}", x, y);

    let x = true;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let x = 2.3;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let x = 'a';
    let y = x;
    println!("x: {}, y: {}", x, y);

    // lets try with some heap data
    let x = String::from("some text");
    let y = x;
    // here we are performing a move
    // not cloning the value into "y"
    // also not making another pointer to "x"'s data (aka shallow copy)
    // rust will invalidate "x"

    // this line will value because x has become invalidated and the value is now moved
    // println!("testing move with x: {}", x);

    // but we can print Y because, it now owns the data
    println!("testing move with y: {}", y);
}

fn takes_ownership(string: String) {
    println!(
        "yay, 'takes_ownership' now has ownership of this text: {}",
        string
    );

    let finesse = string;
    println!(
        "ah but now I, another variable, have ownership: {}",
        finesse
    );
    // println!("ahh no fair, will i create an error? {}", string);
    // yes, you will create an error
}

fn makes_copy(number: i32) {
    let x = number;
    println!("what is number? {}", number);
    println!("what is x? {}", x);
}

fn give_ownership() -> String {
    String::from("Hey, here's a new string for you!")
}

fn take_and_give_ownership(data: String) -> String {
    data
}

fn take_modify_return_ownership(mut string: String) -> String {
    string.push_str(" I've added some new data here :)");
    string
}

fn modify_with_referrence(string: &mut String) {
    string.push_str(" I've added some new data here :)");
}

fn referrences() {
    // our text
    let text: String = String::from("just coding in rust");

    // this method will pass the strings ownership to the parameter in the function
    let length = wrong_calc_length(text);

    // which means this code will not work^
    // println!("the length of: \"{}\" is {} characters long", text, length);

    // this approach is much better
    let text: String = String::from("just coding in rust");
    let length: usize = correct_with_referrence(&text);

    // because TEXT still has ownership over its data since we only passed/created a referrence
    println!("the length of: \"{}\" is {} characters long", text, length);
}

fn wrong_calc_length(text: String) -> usize {
    text.len()
}

fn correct_with_referrence(text: &String) -> usize {
    text.len()
}

fn referrence_constraints() {
    // we can only create one mutable referrence to an owner in a scope
    let mut text: String = String::from("What's up?");

    let mut_ref1 = &mut text;

    // results in an error
    // let mut_ref2 = &mut text;
    // println!("{} {}", mut_ref1, mut_ref2);

    // the benefit of this is that rust can prevent
    // data races at compile time which occurs when we have
    // two pointers on one piece of data and there is no way to sync
    // the pointers about changes to the data

    // we also cannot have mutable and immutable referrences at once

    // we can have multiple immutable data refs because the underlying data will not change
    // and therefore create issues

    /* referrences wrap ups
    1. at any time, i can have 1 mutable referrences or any number of immutable referrences

    2. referrences must always be valid, the data cannot be invalid

    */
}

fn slices() {
    // slices referrence a contiguous sequence of elements within a collection
}
