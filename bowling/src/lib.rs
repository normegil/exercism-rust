#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let frames_nb = self.frames.len();
        if frames_nb > 0 {
            let last_frame = &mut self.frames[frames_nb-1];
            if !last_frame.is_complete() {
                return last_frame.add_second(pins);
            }
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
        if self.frames.len() < 10 {
            return Option::None;
        }
        Option::Some(0)
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

    fn add_second(&mut self, second: u16) -> Result<(), Error> {
        self.second = Option::Some(second);
        Result::Ok({})
    }

    fn is_strike(&self) -> bool {
        self.first == 10 && self.second == Option::None
    }

    fn is_complete(&self) -> bool {
        self.is_strike() || self.second.is_some()
    }
}