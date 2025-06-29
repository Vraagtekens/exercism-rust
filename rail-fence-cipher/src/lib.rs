use std::collections::BTreeMap;

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        let rails: usize = rails.try_into().unwrap();
        RailFence { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut map: BTreeMap<usize, String> = BTreeMap::new();
        let mut rail_count: usize = 1;
        let mut operator: i8 = 1;

        text.chars().for_each(|c| {
            map.entry(rail_count)
                .and_modify(|s| s.push(c))
                .or_insert(c.to_string());

            match operator {
                1 => rail_count += 1,
                -1 => rail_count -= 1,
                _ => rail_count += 1,
            }

            if rail_count == self.rails {
                operator = -1;
            }
            if rail_count == 1 {
                operator = 1;
            }
        });

        map.iter().map(|f| f.1.to_string()).collect::<String>()
    }

    pub fn decode(&self, mut cipher: &str) -> String {
        let map = self.get_map(cipher.len());
        let cipher_length: usize = cipher.len();

        let mut x: BTreeMap<usize, String> = map
            .iter()
            .map(|s| {
                let x = cipher.split_at(s.1.len());
                cipher = x.1;

                (*s.0, x.0.to_string())
            })
            .collect();

        let mut y = String::new();
        let mut rail_count: usize = 1;
        let mut operator: i8 = 1;

        (0..cipher_length).for_each(|_| {
            x.entry(rail_count).and_modify(|s| {
                let b = s.split_at(1);
                y.push_str(b.0);
                *s = b.1.to_string()
            });

            match operator {
                1 => rail_count += 1,
                -1 => rail_count -= 1,
                _ => rail_count += 1,
            }

            if rail_count == self.rails {
                operator = -1;
            }
            if rail_count == 1 {
                operator = 1;
            }
        });

        y
    }

    fn get_map(&self, length: usize) -> BTreeMap<usize, String> {
        let mut map: BTreeMap<usize, String> = BTreeMap::new();
        let mut rail_count: usize = 1;
        let mut operator: i8 = 1;

        (0..length).for_each(|_| {
            map.entry(rail_count)
                .and_modify(|s| s.push('?'))
                .or_insert('?'.to_string());

            match operator {
                1 => rail_count += 1,
                -1 => rail_count -= 1,
                _ => rail_count += 1,
            }

            if rail_count == self.rails {
                operator = -1;
            }
            if rail_count == 1 {
                operator = 1;
            }
        });

        map
    }
}
