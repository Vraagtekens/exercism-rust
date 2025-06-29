use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: BTreeMap<usize, (u16, u16)>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: BTreeMap::new(),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let end = self.frames.get(&9);

        if let Some(e) = end {
            if self.current_frame == 10 {
                if e.0 + e.1 != 10 {
                    return Err(Error::GameComplete);
                }

                if self.frames.contains_key(&10) && e.0 != 10 {
                    return Err(Error::GameComplete);
                }
            }
        }

        let result = self.frames.get(&self.current_frame);

        // println!("{:?}", result);

        match result {
            Some(y) => {
                if y.0 == 10 {
                    self.frames.insert(self.current_frame + 1, (pins, 0));
                } else {
                    self.frames
                        .entry(self.current_frame)
                        .and_modify(|v| v.1 = pins)
                        .or_insert((pins, 0));
                }

                self.current_frame += 1;
            }
            None => {
                self.frames.insert(self.current_frame, (pins, 0));
            }
        }

        // println!("{:?}", self.frames);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // todo!("Return the score if the game is complete, or None if not.");

        if self.frames.is_empty() || self.frames.len() < 10 {
            return None;
        }

        let mut sum: u16 = 0;
        for c in self.frames.iter() {
            if *c.0 == 10 {
                continue;
            }

            if c.1.0 == 10 {
                let first = self.frames.get(&(c.0 + 1)).unwrap_or(&(0, 0));

                sum += c.1.0 + first.0 + first.1;
            } else if c.1.0 + c.1.1 == 10 {
                let first = self.frames.get(&(c.0 + 1)).unwrap_or(&(0, 0));

                sum += c.1.0 + c.1.1 + first.0;
            } else {
                sum += c.1.0 + c.1.1
            }

            println!("{:?}", sum);
        }

        Some(sum)
    }
}
