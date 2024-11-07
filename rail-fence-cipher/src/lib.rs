pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        // todo!("Construct a new fence with {rails} rails")
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        // todo!("Encode this text: {text}")
        // let x: Vec<char> = text.chars().collect();
        let mut vec: Vec<Vec<char>> = vec![];

        for (i, c) in text.chars().enumerate() {
            // if i == self.rails.try_into().unwrap() {}
        }

        "".to_string()
    }

    pub fn decode(&self, cipher: &str) -> String {
        todo!("Decode this ciphertext: {cipher}")
    }
}
