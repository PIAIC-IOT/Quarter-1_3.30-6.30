mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main(){

    

    //absolute path
    crate::front_of_house::hosting::seat_at_table();

    // relative path
    front_of_house::hosting::seat_at_table();

}
