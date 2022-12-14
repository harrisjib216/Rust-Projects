fn main() {
    // printing
    println!("Hello world!");
    println!("I'm learning rust!");

    // comments
    /*
        multi line comments
    */
    let x = 5 + 90 + 5;
    println!("5 + 90 + 5 = {}", x);

    // printing is handled by a series of macros declared in std::fmt
    /*
        format!() writes text into a string
        print!() writes to io::stdout
        println!() writes to the console but appends a newline
        eprint!() same as print!() but the text is printed to io::stderr
        eprintln!() same as the macro above except a newline is appended
    */
    // Note: only types which implement fmt::Display can
    // be formatted with {}
    println!("{} days", 31);

    // this allows us to specify the which item to print based on
    // its index
    println!("{0}, this is {1}.\n{1}, this is {0}.\n{} says \"Hi!\"\n{} also says \"Hi!\"", "Jack", "Jake");

    // arguments can also be named
    println!("{subject} {verb} {object}", object="the lazy dog", verb="jumps over", subject="the quick brown fox");

    // number formatting
    println!("Base 10              {}", 443);
    println!("Base 2               {:b}", 443);
    println!("Base 8               {:o}", 443);
    println!("Base 16              {:x}", 443);
    println!("Base 16 capitalized: {:X}", 443);

    // text padding
    println!("{arg:>5}Greetings!", arg="Seasons"); // right justify
    println!("Seasons Gr{arg:e<10}tings!", arg=""); // pad with characters
    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // here we can capture arguments from a surrounding variable
    let num: f64 = 2.0;
    let width: usize = 11;
    println!("{num:>width$}");

    // extra exercise
    let pi = 3.141592;
    println!("{pi:.precision$}", precision=3);
}