pub mod encryption {

	//implements the caesar cypher on the given text
	pub fn caesar(plaintext: &str, shift: u8) -> String {
		let mut cyphertext: Vec<char> = vec![' '; plaintext.len()];

		for (index, c) in plaintext.chars().enumerate() {
			// checks if c is a-Z
			cyphertext[index] = if c.is_ascii_alphabetic() {
				if c.is_ascii_uppercase() {
					// do upper case shifting 
					((((c as u8) - ('A' as u8)) + shift) % 26 + ('A' as u8)) as char
				} else {
					// do lower case shifting
					((((c as u8) - ('a' as u8)) + shift) % 26 + ('a' as u8)) as char
				}
			} else {
				c
			};
		}

		cyphertext.into_iter().collect()
	}
}

