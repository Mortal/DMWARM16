use std::{io, fmt, result, error, str, num};

#[derive(Debug)]
pub enum WordsError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    UnexpectedEof,
}

impl fmt::Display for WordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordsError::Io(ref err) => write!(f, "IO error: {}", err),
            WordsError::ParseInt(ref err) => write!(f, "ParseInt error: {}", err),
            WordsError::ParseFloat(ref err) => write!(f, "ParseFloat error: {}", err),
            WordsError::UnexpectedEof => write!(f, "Unexpected end-of-file"),
        }
    }
}

/*
// Not allowed in Rust 1.0 stable
impl error::Error for WordsError {
    fn description(&self) -> &str {
        match *self {
            WordsError::Io(ref err) => err.description(),
            WordsError::ParseInt(ref err) => err.description(),
            WordsError::ParseFloat(ref err) => err.description(),
            WordsError::UnexpectedEof => "Unexpected end-of-file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WordsError::Io(ref err) => Some(err),
            WordsError::ParseInt(ref err) => Some(err),
            WordsError::ParseFloat(ref err) => Some(err),
            WordsError::UnexpectedEof => None,
        }
    }
}
*/

impl From<io::Error> for WordsError {
    fn from(err: io::Error) -> WordsError {
        WordsError::Io(err)
    }
}

impl From<num::ParseIntError> for WordsError {
    fn from(err: num::ParseIntError) -> WordsError {
        WordsError::ParseInt(err)
    }
}

impl From<num::ParseFloatError> for WordsError {
    fn from(err: num::ParseFloatError) -> WordsError {
        WordsError::ParseFloat(err)
    }
}

#[derive(Debug)]
pub struct Words<R: io::BufRead> {
    reader: R,
}

pub type Result<T> = result::Result<T, WordsError>;

fn is_ws(c: char) -> bool { c == '\n' || c == ' ' }

impl<R: io::BufRead> Words<R> {
    pub fn new(reader: R) -> Self {
        Words {
            reader: reader,
        }
    }

    fn next_ascii_char(&mut self) -> Result<Option<char>> {
        let mut buf = [0; 1];
        let n = try!(self.reader.read(&mut buf));
        if n == 0 { return Ok(None); }
        Ok(Some(match str::from_utf8(&buf) {
            Ok(s) => s.chars().next().unwrap(),
            Err(_) => '\0',  // not ASCII
        }))
    }

    pub fn next(&mut self) -> Result<Option<String>> {
        let mut s = String::new();
        loop {
            match try!(self.next_ascii_char()) {
                Some(c) => if is_ws(c) { if s.len() > 0 { break }; } else { s.push(c) },
                None => break,
            }
        }
        Ok(if s.len() == 0 { None } else { Some(s) })
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut s = String::new();
        try!(self.reader.read_line(&mut s));
        Ok(s)
    }

    pub fn parse_next<T: str::FromStr>(&mut self) -> Result<T>
        where WordsError: From<<T as str::FromStr>::Err>
    {
        match try!(self.next()) {
            None => return Err(WordsError::UnexpectedEof),
            Some(word) => return Ok(try!(word.parse())),
        }
    }
}
