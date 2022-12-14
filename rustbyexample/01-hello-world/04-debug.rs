// making this struct printable
#[derive(Debug)]
struct OriginallyUnPrintable(i32);

// making a clone of the struct printable as well
#[derive(Debug)]
struct Clone(OriginallyUnPrintable);

#[derive(Debug)]
struct Person {
    age: u8,
    inch_height: u8,
}

fn main() {
    println!("{:?} is printable", OriginallyUnPrintable(23));
    println!("{:#?} as well", Clone(OriginallyUnPrintable(22)));

    println!("{:?}", Person { age: 25, inch_height: 2 });
    println!("{:#?}", Person { age: 25, inch_height: 2 });
}