use std::collections::HashMap;

/*
collections are created in the heap which means
collections can grow or shrink dynamically
memory has to be allocated to store values/data

*/
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // fixed and sized array
    let arr: [u8; 4] = [1, 2, 3, 4];

    // heap array or vector
    let vector: Vec<u8> = Vec::new();

    // the type is not defined so this syntax is illegal
    // let invalid = Vec::new();

    // although we have a dynamically sized and typed vector
    // our vector is still immutalbe by default, so to modify
    // our vector, we must make it mutable
    // vector.push(3);

    let mut vector: Vec<u8> = Vec::new();
    vector.push(23);

    // lets create a vector and initialize it with values
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    vector[2] = 1;

    println!("vector contents: {:?}", vector);

    let third = &vector[2];
    println!("third element: {}", third);
    println!("vector contents: {:?}", vector);

    let p = vector.get(3);
    match p {
        Some(val) => println!("index is in bounds"),
        None => println!("index is out of bounds"),
    };

    strings();
    hash_maps();
}

fn strings() {
    // strings are a collection of UTF-8 encoded bytes
    // unicode is the universal character set that represents all the
    // well known characters
    // with ASCII each character is one byte, with UTF 8 each character
    // can have 1-4 bytes

    let string_slice = "the dog";
    let owned_string = String::from("look up above");

    let string_slice = "✅"; // UTF-8 emojies
    let owned_string = String::from("привет!");

    let mut owned_string = owned_string;
    owned_string.push('v');

    // concat strings
    let str1 = String::from("abc");
    let str2 = String::from("efg");
    let str3 = str1 + &str2;

    // or
    let str3 = format!("{}{}", str2, str3);

    // extracting bytes/data at indices
    let hello = String::from("Здравствуйте"); // 24 bytes
    let hello = String::from("नमस्ते"); // 18 bytes

    // for "नमस्ते" or namaste
    // bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for byte_value in hello.bytes() {
        print!("{}, ", byte_value);
    }
    print!("\n");

    // scalar values -> possible partial characters
    // ['न', 'म', 'स', '्', 'त', 'े']
    for scalar_chars in hello.chars() {
        print!("{}, ", scalar_chars);
    }
    print!("\n");

    // grapheme clusters
    // ["न", "म", "स्", "ते"]
    for humanable_char in hello.graphemes(true) {
        print!("{}, ", humanable_char);
    }
    print!("\n");
}

fn hash_maps() {
    let red = String::from("red");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, u8> = HashMap::new();

    // note: scores will now have onwership of these two variables
    scores.insert(red, 30);
    scores.insert(yellow, 15);
    // println("{}", red);

    // how to get values from a hashmap
    let team_name = String::from("easy");
    match scores.get(&team_name) {
        Some(score) => println!("{} scored {} points!", score, team_name),
        None => println!("eh? sorry this team doesn't exist"),
    }

    scores.insert(String::from("green"), 23);
    let start_mut_ref = scores.entry(String::from("star")).or_insert(2);
    *start_mut_ref += 3;

    // print keys of hash map
    println!("{:?}", scores);
}
