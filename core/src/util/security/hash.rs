use ring::hmac;

pub fn hash(input: &str) -> String {
    let key = std::env::var("HASH_KEY").expect("HASH_KEY must be set");

    // Create an HMAC-SHA256 context using the provided key
    let key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
    let hmac_result = hmac::sign(&key, input.as_bytes());

    // Get the bytes of the HMAC result and convert them to a hexadecimal string
    let hash_bytes = hmac_result.as_ref();
    let hex_hash = hash_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    hex_hash
}

mod tests {
    #[test]
    fn test_hash() {
        std::env::set_var("HASH_KEY", "test");
        let input = "hello world";
        let expected = "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d922";

        let actual = super::hash(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_different_key() {
        std::env::set_var("HASH_KEY", "test2");
        let input = "hello world";

        let actual = super::hash(input);

        assert_ne!(
            actual,
            "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d922"
        );
    }
}
