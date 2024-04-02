// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

fn take_family_photo() {
    //
}

// creating modules
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {
            add_to_waitlist();
            crate::take_family_photo();
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// paths
pub fn eat_at_restaurant() {
    // absolute and relative paths
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // a module or code in a module needs to be public to be used
    // this code will cause an error
    // front_of_house::serving::take_order()
    // by default a child module and everything inside of it is private
}

// privacy rules with structs
mod back_of_house {
    pub struct Breakfast {
        pub entre: String,
        quantity: u8,
    }

    impl Breakfast {
        pub fn make_meal(entre: String) -> Breakfast {
            Breakfast { entre, quantity: 1 }
        }
    }
}

pub fn decide_from_menu() {
    let mut my_meal = crate::back_of_house::Breakfast::make_meal(String::from("pancakes"));

    my_meal.entre = String::from("yogurt");

    // this will not work since fields are private by default
    // my_meal.quantity = 32;
}

//
mod dinning_hall;

pub use crate::dinning_hall::seating;

pub fn get_seated() {
    seating::set_the_table();
}
