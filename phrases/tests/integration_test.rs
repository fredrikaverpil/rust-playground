extern crate phrases;

// Integration tests
#[cfg(test)]
mod test {

    #[test]
    fn english_greeting_is_correct() {
        assert_eq!("hello", phrases::greetings::english::hello());
    }

    #[test]
    #[should_panic]
    fn english_greeting_should_panic() {
        assert_eq!("hellooooo", phrases::greetings::english::hello());
    }

    #[test]
    #[ignore]
    fn ignore_me() {
        // do nothing
    }
}