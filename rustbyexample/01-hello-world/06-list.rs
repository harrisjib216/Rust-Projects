use std::fmt::{Display, Formatter, Result};

struct List(Vec<u8>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;

        let last_index = self.0.len();

        for (index, val) in self.0.iter().enumerate() {
            write!(f, "{}", val);

            if index < last_index-1 {
                write!(f, ", ");
            }
        }

        write!(f, "]")
    }
}

fn main() {
    let list = List(vec![]);
    println!("No items\n>{}", list);

    let list = List(vec![1]);
    println!("One item\n>{}", list);

    let list = List(vec![1,2,3,4]);
    println!("Multiple items\n>{}", list);
}