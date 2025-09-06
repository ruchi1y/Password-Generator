mod generator;
mod user_interaction;
fn main() {
    let password_length = user_interaction::read_menu_option() as usize;
    let pwd = generator::password_generator(password_length);
    println!("Your password is: {}", pwd);

    let level = generator::security_level(&pwd);
    print!("Security level: {}\n", level);
}
