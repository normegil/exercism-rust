#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let frames_nb = self.frames.len();
        let mut allow_new_frame = frames_nb < 10;
        if frames_nb > 0 {
            let previous_frame_is_strike = if frames_nb <= 1 { false } else { self.frames[frames_nb-2].is_strike() };
            let last_frame = &mut self.frames[frames_nb-1];
            if !last_frame.is_complete() {
                if frames_nb > 10 && !previous_frame_is_strike {
                    return Result::Err(Error::GameComplete);
                }
                return last_frame.add_second(pins);
            } else if last_frame.is_spare() {
                allow_new_frame = true;
            } else if last_frame.is_strike() {
                allow_new_frame = true;
            }
        }

        if !allow_new_frame {
            return Result::Err(Error::GameComplete);
        }

        match Frame::new(pins) {
            Result::Ok(frame) => {
                self.frames.push(frame);
                return Result::Ok(())
            },
            Result::Err(err) => return Result::Err(err),
        }
    }

    pub fn score(&self) -> Option<u16> {
        let frames_nb = self.frames.len();
        if frames_nb < 10 {
            return Option::None;
        } else if frames_nb == 10 && self.frames[frames_nb-1].is_strike() {
            return Option::None;
        } else if frames_nb == 11 && self.frames[frames_nb-1].is_strike() && self.frames[frames_nb-2].is_strike() {
            return Option::None;
        }

        let mut score = 0;
        for (index, frame) in self.frames.iter().enumerate() {
            let is_extra_frame = index >= 10;
            if frame.is_strike() && !is_extra_frame {
                score += 10;
                let next_frame = &self.frames[index + 1];
                if !next_frame.is_strike() || index + 1 == 11 {
                    score += next_frame.first_throw_score() + next_frame.second_throw_score()
                } else {
                    let next_next_frame = &self.frames[index + 2];
                    score += 10 + next_next_frame.first_throw_score();
                }
            } else if frame.is_spare() && !is_extra_frame{
                score += 10;
                if index == 9 {
                    score += self.frames[index + 1].first_throw_score()
                } else {
                    score += self.frames[index + 1].first_throw_score()
                }
            } else if !is_extra_frame {
                score += frame.first_throw_score() + frame.second_throw_score();
            }
        }
        Option::Some(score)
    }
}

struct Frame {
    first: u16,
    second: Option<u16>
}

impl Frame {
    fn new(first: u16) -> Result<Self, Error> {
        if first > 10 {
            return Result::Err(Error::NotEnoughPinsLeft);
        }
        Result::Ok(Frame { first, second: Option::None })
    }

    fn first_throw_score(&self) -> u16 {
        return self.first;
    }

    fn second_throw_score(&self) -> u16 {
        match self.second {
            Option::None => return 0,
            Option::Some(second) => second
        }
    }

    fn add_second(&mut self, second: u16) -> Result<(), Error> {
        if self.first + second > 10 {
            return Result::Err(Error::NotEnoughPinsLeft);
        }
        self.second = Option::Some(second);
        Result::Ok({})
    }

    fn is_strike(&self) -> bool {
        self.first == 10 && self.second == Option::None
    }

    fn is_spare(&self) -> bool {
        if self.is_strike() {
            return false;
        }
        if self.second.is_none() {
            return false;
        }
        return self.first + self.second.unwrap() == 10
    }

    fn is_complete(&self) -> bool {
        self.is_strike() || self.second.is_some()
    }
}