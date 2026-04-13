fn print_name(name: &String) {
    println!("name: {}", name);
}

fn add_exclamation(text: &mut String) {
    text.push_str("!");
}

fn main() {
    let name = String::from("shiba");
    print_name(&name);
    println!("main: {}", name);

    let mut message = String::from("hello");
    add_exclamation(&mut message);
    println!("message: {}", message);
}
