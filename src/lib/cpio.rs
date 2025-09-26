use std::fmt::Formatter;
use std::{
    fmt::{Debug, Display},
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, StdinLock, Write},
    iter::FromIterator,
    str::FromStr,
    convert::TryInto,
};
pub use CPResult::*;

pub enum CPResult<T, E>
where
    T: Display,
    E: Display,
{
    Success(T),
    Failure(E),
}

#[macro_export]
macro_rules! unwrap {
    ($value:expr) => {
        match $value {
            Ok(v) => v,
            Err(e) => return Failure(e),
        }
    };
}

impl<T, E> From<Result<T, E>> for CPResult<T, E>
where
    T: Display,
    E: Display,
{
    fn from(value: Result<T, E>) -> Self {
        use CPResult::*;
        match value {
            Ok(v) => Success(v),
            Err(e) => Failure(e),
        }
    }
}

impl<S, T> Display for CPResult<S, T>
where
    S: Display,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Success(v) => write!(f, "{v}"),
            Failure(v) => write!(f, "{v}"),
        }
    }
}

pub struct Boolean<const CASE: u8>(bool);

pub type Bool = Boolean<0>;
pub type BOOL = Boolean<1>;

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

impl<const CASE: u8> Display for Boolean<CASE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = if self.0 { "Yes" } else { "No" };
        if CASE == 1 {
            write!(f, "{}", res.to_uppercase())
        } else {
            write!(f, "{res}")
        }
    }
}

/// A generic list with configurable separator for formatting
pub struct ListOf<const SEP: char, T>(pub Vec<T>);

/// Space-separated list
pub type Words<T> = ListOf<' ', T>;

/// Newline-separated list
pub type Lines<T> = ListOf<'\n', T>;

/// Create a space-separated list
pub fn words_of<T>(v: Vec<T>) -> Words<T> {
    ListOf(v)
}

/// Create a newline-separated list
pub fn lines_of<T>(v: Vec<T>) -> Lines<T> {
    ListOf(v)
}

impl<T, const S: char> From<Vec<T>> for ListOf<S, T> {
    fn from(value: Vec<T>) -> Self {
        ListOf(value)
    }
}

impl<R, const S: char> FromIterator<R> for ListOf<S, R> {
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

/// Buffered input reader for competitive programming
pub struct CPInput<R: Read> {
    reader: BufReader<R>,
    buf: String,
    eof_reached: bool
}

impl<T, const SEP: char> Display for ListOf<SEP, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        if SEP == '\0' {
            // No separator
            for item in &self.0 {
                write!(f, "{}", item)?;
            }
        } else {
            // With separator
            let mut iter = self.0.iter();
            if let Some(first) = iter.next() {
                write!(f, "{}", first)?;
                for item in iter {
                    write!(f, "{}{}", SEP, item)?;
                }
            }
        }
        Ok(())
    }
}


// Add these methods to CPInput implementation
impl<R> CPInput<R>
where
    R: Read,
{
    /// Create a new CPInput with the given reader
    pub fn new(input: R) -> Self {
        CPInput {
            reader: BufReader::new(input),
            buf: String::with_capacity(1024),
            eof_reached: false,
        }
    }

    /// Check if EOF has been reached
    pub fn is_eof(&self) -> bool {
        self.eof_reached
    }

    /// Try to read a line, returning None if EOF is reached
    pub fn try_read_line<F, T>(&mut self, parser: F) -> Result<Option<T>, String>
    where
        F: Fn(&str) -> Result<T, String>,
    {
        if self.eof_reached {
            return Ok(None);
        }

        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(0) => {
                self.eof_reached = true;
                Ok(None)
            }
            Ok(_) => {
                let trimmed = self.buf.trim();
                if trimmed.is_empty() {
                    // Empty line, try next line
                    self.try_read_line(parser)
                } else {
                    parser(trimmed).map(Some)
                }
            }
            Err(err) => Err(format!("IO error: {}", err)),
        }
    }

    /// Try to read multiple lines, returning None if EOF is reached
    pub fn try_read_lines<F, T>(&mut self, n: usize, parser: F) -> Result<Option<Vec<T>>, String>
    where
        F: Fn(&str) -> Result<T, String>,
    {
        if self.eof_reached {
            return Ok(None);
        }

        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            match self.try_read_line(&parser) {
                Ok(Some(value)) => result.push(value),
                Ok(None) => {
                    if i == 0 {
                        // EOF on first line
                        return Ok(None);
                    } else {
                        // EOF in the middle - this is an error
                        return Err(format!("Unexpected EOF after reading {} of {} lines", i, n));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(Some(result))
    }

    /// Read a single line and parse it using the provided parser
    pub fn read_line<F, T>(&mut self, parser: F) -> Result<T, String>
    where
        F: Fn(&str) -> Result<T, String>,
    {
        self.buf.clear();
        self.reader
            .read_line(&mut self.buf)
            .map_err(|err| format!("IO error: {}", err))?;
        parser(self.buf.trim())
    }

    /// Read multiple lines and parse each using the provided parser
    pub fn read_lines<F, T>(&mut self, n: usize, parser: F) -> Result<Vec<T>, String>
    where
        F: Fn(&str) -> Result<T, String>,
    {
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            self.buf.clear();
            self.reader
                .read_line(&mut self.buf)
                .map_err(|err| format!("IO error on line {}: {}", i + 1, err))?;
            result.push(parser(self.buf.trim())?);
        }
        Ok(result)
    }
}



/// Solve problems that read until EOF
pub fn solve_eof<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> Option<O>)
where
    O: Display,
{
    let mut input = CPInput::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    
    while !input.is_eof() {
        if let Some(output) = solution(&mut input) {
            write!(writer, "{output}\n").unwrap();
        } else {
            break;
        }
    }
    writer.flush().unwrap();
}


/// Parse a single value from string
pub fn parse<T: FromStr>(s: &str) -> Result<T, String>
where
    T::Err: ToString,
{
    s.parse::<T>()
        .map_err(|err| format!("Failed to parse '{}': {}", s, err.to_string()))
}

/// Parse space-separated values from string
pub fn parse_vec<T: FromStr>(s: &str) -> Result<Vec<T>, String>
where
    T::Err: ToString,
{
    s.split_whitespace().map(|token| parse(token)).collect()
}

/// Parse space-separated values into fixed-size array
pub fn parse_array<T: FromStr + Debug, const N: usize>(s: &str) -> Result<[T; N], String>
where
    T::Err: ToString,
{
    let vec: Vec<T> = parse_vec(s)?;
    vec.try_into()
        .map_err(|v: Vec<T>| format!("Expected {} elements, got {}", N, v.len()))
}

/// Parse string into vector of characters
pub fn parse_chars(s: &str) -> Result<Vec<char>, String> {
    Ok(s.chars().collect())
}

/// Parse binary string (0s and 1s) into vector of u8
pub fn parse_binary(s: &str) -> Result<Vec<u8>, String> {
    s.chars()
        .map(|c| match c {
            '0' => Ok(0),
            '1' => Ok(1),
            _ => Err(format!("Invalid binary character: '{}'", c)),
        })
        .collect()
}

/// Solve a single test case problem
pub fn solve<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> O)
where
    O: Display,
{
    let mut input = CPInput::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let output = solution(&mut input);
    write!(writer, "{output}\n").unwrap();
    writer.flush().unwrap()
}

/// Solve multiple test cases problem
pub fn solve_n<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> O)
where
    O: Display,
{
    let mut input = CPInput::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let n: usize = input.read_line(parse).unwrap();
    for _ in 0..n {
        let output = solution(&mut input);
        write!(writer, "{output}\n").unwrap();
    }
    writer.flush().unwrap();
}

// Macro helpers for main function generation
#[macro_export]
macro_rules! sol_main {
    ($solve_fn:ident, $name:ident) => {
        fn main() {
            $solve_fn($name);
        }
    };
}

/// Macro for single test case problems
#[macro_export]
macro_rules! sol {
    (
        fn $name:ident (
            $(
               $var:tt: $ty:tt $(; $n:expr)?
            ),* $(,)?
        ) -> $ret:ty
        $body:block
    ) => {
        sol_main!(solve, $name);

        fn $name<R>(input: &mut CPInput<R>) -> $ret
        where
            R: std::io::Read, {
            $(
                read_value!(input, $var, $ty $(; $n)?);
            )*
            $body
        }
    };
}

/// Macro for multiple test case problems
#[macro_export]
macro_rules! sol_n {
    (
        fn $name:ident (
            $(
               $var:tt: $ty:tt $(; $n:expr)?
            ),* $(,)?
        ) -> $ret:ty
        $body:block
    ) => {
        sol_main!(solve_n, $name);

        fn $name<R>(input: &mut CPInput<R>) -> $ret
        where
            R: std::io::Read, {
            $(
                read_value!(input, $var, $ty $(; $n)?);
            )*
            $body
        }
    };
}


/// Macro for EOF problems (read until end of file)
#[macro_export]
macro_rules! sol_eof {
    (
        fn $name:ident (
            $(
               $var:tt: $ty:tt $(; $n:expr)?
            ),* $(,)?
        ) -> $ret:ty
        $body:block
    ) => {
        sol_main!(solve_eof, $name);

       fn $name<R>(input: &mut CPInput<R>) -> Option<$ret>
        where
            R: std::io::Read, 
        {
            $(
                try_read_value!(input, $var, $ty $(; $n)?);
            )*
            Some($body)
        }
    };
}

/// Macro for reading input values with various patterns
#[macro_export]
macro_rules! read_value {
    // 2D character grid
    ($input:ident, $var:tt, [[char]; $n:expr]) => {
        let $var: Vec<Vec<char>> = $input.read_lines($n, parse_chars).unwrap();
    };

    // 2D grid with fixed sized
    ($input:ident, $var:tt, [[$inner:ty; $N:literal]; $n:expr]) => {
        let $var: Vec<[$inner; $N]> = $input.read_lines($n, parse_array).unwrap();
    };

    // 2D binary grid
    ($input:ident, $var:tt, [[01]; $n:expr]) => {
        let $var: Vec<Vec<u8>> = $input.read_lines($n, parse_binary).unwrap();
    };

    // 2D grid
    ($input:ident, $var:tt, [[$inner:ty]; $n:expr]) => {
        let $var: Vec<Vec<$inner>> = $input.read_lines($n, parse_vec).unwrap();
    };

    // Multiple lines of single values
    ($input:ident, $var:tt, [$inner:ty]; $n:expr) => {
        let $var: Vec<$inner> = $input.read_lines($n, parse).unwrap();
    };

    // Fixed-size array from single line
    ($input:ident, $var:tt, [$inner:ty; $N:literal]) => {
        let $var: [$inner; $N] = $input.read_line(parse_array).unwrap();
    };

    // Single line of characters
    ($input:ident, $var:tt, [char]) => {
        let $var: Vec<char> = $input.read_line(parse_chars).unwrap();
    };

    // Single line of binary
    ($input:ident, $var:tt, [01]) => {
        let $var: Vec<u8> = $input.read_line(parse_binary).unwrap();
    };

    // Vector from single line
    ($input:ident, $var:tt, [$inner:ty]) => {
        let $var: Vec<$inner> = $input.read_line(parse_vec).unwrap();
    };

    // Single value
    ($input:ident, $var:tt, $inner:ty) => {
        let $var: $inner = $input.read_line(parse).unwrap();
    };
}

// Helper macro for trying to read values with EOF detection
#[macro_export]
macro_rules! try_read_value {
    // 2D character grid
    ($input:ident, $var:tt, [[char]; $n:expr]) => {
        let $var: Vec<Vec<char>> = match $input.try_read_lines($n, parse_chars) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // 2D grid with fixed sized
    ($input:ident, $var:tt, [[$inner:ty; $N:literal]; $n:expr]) => {
        let $var: Vec<[$inner; $N]> = match $input.try_read_lines($n, parse_array) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // 2D binary grid
    ($input:ident, $var:tt, [[01]; $n:expr]) => {
        let $var: Vec<Vec<u8>> = match $input.try_read_lines($n, parse_binary) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // 2D grid
    ($input:ident, $var:tt, [[$inner:ty]; $n:expr]) => {
        let $var: Vec<Vec<$inner>> = match $input.try_read_lines($n, parse_vec) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Multiple lines of single values
    ($input:ident, $var:tt, [$inner:ty]; $n:expr) => {
        let $var: Vec<$inner> = match $input.try_read_lines($n, parse) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Fixed-size array from single line
    ($input:ident, $var:tt, [$inner:ty; $N:literal]) => {
        let $var: [$inner; $N] = match $input.try_read_line(parse_array) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Single line of characters
    ($input:ident, $var:tt, [char]) => {
        let $var: Vec<char> = match $input.try_read_line(parse_chars) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Single line of binary
    ($input:ident, $var:tt, [01]) => {
        let $var: Vec<u8> = match $input.try_read_line(parse_binary) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Vector from single line
    ($input:ident, $var:tt, [$inner:ty]) => {
        let $var: Vec<$inner> = match $input.try_read_line(parse_vec) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
    
    // Single value
    ($input:ident, $var:tt, $inner:ty) => {
        let $var: $inner = match $input.try_read_line(parse) {
            Ok(Some(v)) => v,
            Ok(None) => return None,
            Err(_) => return None,
        };
    };
}