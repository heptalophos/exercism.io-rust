pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rail: u32 = 0u32;
        let mut up: bool = false;
        let mut fence: Vec<String> = Vec::<String>::new();
        for _ in 0..self.rails { fence.push("".to_string()) }
        for c in text.chars() {
            fence[rail as usize].push(c);
            match up {
                true => {
                    if rail == 0 {
                        rail += 1u32;
                        up = false;
                    } 
                    else {
                        rail -= 1u32;
                    }
                }
                false => {
                    if rail == self.rails - 1 {
                        rail -= 1u32;
                        up = true;
                    } 
                    else {
                        rail += 1u32;
                    }
                }
            };
        }
        let mut cipher = String::new();
        for i in 0..self.rails {
            cipher.push_str(&fence[i as usize]);
        }
        cipher
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail: u32 = 0u32;
        let mut up: bool = false;
        let mut fence: Vec<Vec<char>> = Vec::<Vec<char>>::new();
        for _ in 0..self.rails { fence.push(vec![]) }
        for _ in cipher.chars() {
            fence[rail as usize].push('?');
            match up {
                true => {
                    if rail == 0u32 {
                        rail += 1u32;
                        up = false;
                    } 
                    else {
                        rail -= 1u32;
                    }
                }
                false => {
                    if rail == self.rails - 1 {
                        rail -= 1u32;
                        up = true;
                    } 
                    else {
                        rail += 1u32;
                    }
                }
            };
        }
        let mut rail: u32 = 0u32;
        let mut position = 0;
        for c in cipher.chars() {
            fence[rail as usize][position] = c;
            position += 1;
            if position == fence[rail as usize].len() {
                position = 0;
                rail += 1u32;
            }
        }
        let mut plain: String = String::new();
        let mut rail: u32 = 0u32;
        let mut up: bool = false;
        let mut p: Vec<usize> = Vec::<usize>::new();
        for _ in 0..fence.len() { p.push(0) }
        for _ in cipher.chars() {
            plain.push(fence[rail as usize][p[rail as usize] as usize]);
            p[rail as usize] += 1;
            match up {
                true => {
                    if rail == 0 {
                        rail += 1u32;
                        up = false;
                    } 
                    else {
                        rail -= 1u32;
                    }
                }
                false => {
                    if rail == self.rails - 1 {
                        rail -= 1u32;
                        up = true;
                    } 
                    else {
                        rail += 1u32;
                    }
                }
            }
        }
        plain
    }
}
