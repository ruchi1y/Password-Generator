use std::io;

pub fn read_menu_option() -> u32 {
    loop {
        let mut input = String::new();
        println!("Insert a number:");

        io::stdin()
            .read_line(&mut input)
            .expect("ERROR");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Wrong input, insert a number."),
        }
    }
}