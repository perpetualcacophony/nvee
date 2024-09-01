pub struct Parser<'i> {
    input: &'i str,
}

impl<'i> Parser<'i> {
    pub(super) fn new(input: &'i str) -> Self {
        Self { input }
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().next()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Some(ch)
    }

    pub fn parse<P: Parse>(&mut self) -> Result<P, P::Err> {
        P::parse(self)
    }
}

pub trait Parse: Sized {
    type Err;

    fn parse(input: &mut Parser<'_>) -> Result<Self, Self::Err>;
}

impl Parse for char {
    type Err = ();

    fn parse(input: &mut Parser<'_>) -> Result<Self, Self::Err> {
        input.next_char().ok_or(())
    }
}

impl Parse for u64 {
    type Err = std::num::ParseIntError;

    fn parse(input: &mut Parser<'_>) -> Result<Self, Self::Err> {
        let mut digits = String::new();

        while let Some(digit @ '0'..='9') = input.next_char() {
            digits.push(digit)
        }

        digits.parse()
    }
}
