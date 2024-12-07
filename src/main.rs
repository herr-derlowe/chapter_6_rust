mod chapter6;
fn main() {
    println!("Hello, world!");
    chapter6::call_message();
    chapter6::use_option();

    chapter6::guess();
    chapter6::value_in_cents(chapter6::Coin::Quarter(chapter6::UsState::Alabama));
    let x = Some(-5);
    println!("This is {} times two {}", x.unwrap(), chapter6::times_two(x).unwrap());

    chapter6::use_if_let();
}
