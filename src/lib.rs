#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Decoder {
    pub total_frames: u64,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder { total_frames: 0 }
    }

    pub fn parse(&mut self) {
        self.total_frames += 1;
    }
}
