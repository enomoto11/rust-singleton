use my_module::singleton_integer;

fn main() {
    let value = singleton_integer().value();
    println!("The value is {}", value);
}
