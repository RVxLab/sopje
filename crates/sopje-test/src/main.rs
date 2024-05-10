fn main() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    #[sopje::sdk()]
    struct Sdk;

    #[test]
    pub fn it_works() {
        assert_eq!("Hello!", Sdk::say_hello());
    }
}
