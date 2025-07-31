use std::fmt::Formatter;
use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, StdinLock, Write},
    iter::FromIterator,
    str::FromStr,
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
            Err(e) => return Failure(e)
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
    fn from_iter<T: IntoIterator<Item=R>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

/// Buffered input reader for competitive programming
pub struct CPInput<R: Read> {
    reader: BufReader<R>,
    buf: String,
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

impl<R> CPInput<R>
where
    R: Read,
{
    /// Create a new CPInput with the given reader
    pub fn new(input: R) -> Self {
        CPInput {
            reader: BufReader::new(input),
            buf: String::with_capacity(1024), // Pre-allocate for performance
        }
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

// Debug utilities (only in debug builds)
#[macro_export]
macro_rules! debug {
    ($($arg:expr),* $(,)?) => {
        eprintln!($($arg),*);
    };
}
