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

    pub fn parse_char(&mut self, ch: char) -> bool {
        self.parse_char_with(|next| next == ch).is_some()
    }

    pub fn parse_char_with(&mut self, matches: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(next) = self.peek_char() {
            if matches(next) {
                self.next_char();
                return Some(next);
            }
        }

        None
    }

    pub fn parse<P: Parse>(&mut self) -> Result<P, P::Err> {
        P::parse(self)
    }
}

pub trait Sealed {}

pub trait Parse: Sized + Sealed {
    type Err;

    fn parse(input: &mut Parser<'_>) -> Result<Self, Self::Err>;

    fn parse_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse()
    }
}

impl Sealed for u64 {}
impl Parse for u64 {
    type Err = std::num::ParseIntError;

    fn parse(input: &mut Parser<'_>) -> Result<Self, Self::Err> {
        let mut digits = String::new();

        while let Some(digit) = input.parse_char_with(|ch| ch.is_ascii_digit()) {
            digits.push(digit)
        }

        digits.parse()
    }
}
