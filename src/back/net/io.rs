use Error;
use mio;

/// Handles IO.
pub struct Io
{
    pub poll: mio::Poll,
    pub events: mio::Events,
    /// An accumulator used to give us unique mio::Token objects.
    token_accumulator: usize,
}

impl Io
{
    pub fn new() -> Result<Self, Error> {
        Ok(Io {
            poll: mio::Poll::new()?,
            events: mio::Events::with_capacity(1024),
            token_accumulator: 0,
        })
    }

    /// Generates a unique MIO token.
    pub fn create_token(&mut self) -> mio::Token {
        let token = mio::Token(self.token_accumulator);
        self.token_accumulator += 1;
        token
    }
}
