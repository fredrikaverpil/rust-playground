pub mod greetings {

    pub mod english; // looks at english.rs

    pub mod french {
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn bye() -> String { "au revoir".to_string() }
    }

}


// Unit tests
#[cfg(test)]
mod test {

    use super::greetings;

    #[test]
    fn english_greeting_is_correct() {
        assert_eq!("hello", greetings::english::hello());
    }

    #[test]
    #[should_panic]
    fn english_greeting_should_panic() {
        assert_eq!("hellooooo", greetings::english::hello());
    }

    #[test]
    #[ignore]
    fn ignore_me() {
        // do nothing
    }

}