use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: BTreeMap<usize, Vec<u16>>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 1,
            frames: BTreeMap::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if let Some(frame10) = self.frames.get(&10) {
            if frame10.iter().sum::<u16>() != 10 && frame10.len() == 2 {
                return Err(Error::GameComplete);
            }

            let frame11 = self.frames.get(&11).map_or(0, |v| v.len());

            let x = frame10.first().unwrap();
            if x == &10 {
                // strike
                if frame11 == 2 {
                    return Err(Error::GameComplete);
                }
            } else {
                // spare
                if frame11 == 1 {
                    return Err(Error::GameComplete);
                }
            }
        }

        let result = self.frames.get(&self.current_frame);
        if let Some(y) = result {
            let sum: u16 = y.iter().sum();
            if sum == 10 || y.len() == 2 {
                self.current_frame += 1;
            }
        }

        self.frames
            .entry(self.current_frame)
            .or_default()
            .push(pins);

        let x = self.frames.get(&self.current_frame).unwrap();
        if x.iter().sum::<u16>() > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.is_empty() || self.frames.len() < 10 {
            return None;
        }

        self.count()
    }

    fn count(&self) -> Option<u16> {
        let mut sum: u16 = 0;

        for (index, frame) in self.frames.iter() {
            if index == &11 {
                break;
            }

            let points: u16 = frame.iter().sum();

            if points == 10 {
                let next = self.frames.get(&(index + 1))?;

                if 10 == *frame.first()? {
                    // Strike
                    if 10 == *next.first()? {
                        let next2 = self.frames.get(&(index + 2))?;

                        sum += next.first()? + next2.first()?;
                    } else {
                        sum += next.iter().sum::<u16>();
                    }
                } else {
                    // Spare
                    sum += next.first()?;
                }
            }

            sum += points;
        }

        Some(sum)
    }
}
