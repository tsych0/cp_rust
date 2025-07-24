use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, StdinLock, Write},
    iter::FromIterator,
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, String>;

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

#[derive(Debug, Clone, Copy)]
pub struct Bool(bool);

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        Bool(b)
    }
}

impl CPFormat for Bool {
    fn cp_fmt(self) -> String {
        if self.0 { "YES" } else { "NO" }.to_string()
    }
}

/// Trait for formatting output in competitive programming style
pub trait CPFormat {
    fn cp_fmt(self) -> String;
}

impl<T, Q> CPFormat for std::result::Result<T, Q>
where
    T: CPFormat,
    Q: CPFormat,
{
    fn cp_fmt(self) -> String {
        match self {
            Ok(t) => t.cp_fmt(),
            Err(e) => e.cp_fmt(),
        }
    }
}

impl CPFormat for bool {
    fn cp_fmt(self) -> String {
        (if self { "Yes" } else { "No" }).to_string()
    }
}

impl<T, const SEP: char> Display for ListOf<SEP, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<T, const SEP: char> CPFormat for ListOf<SEP, T>
where
    T: CPFormat,
{
    fn cp_fmt(self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        if SEP == '\0' {
            // No separator - concatenate directly
            self.0.into_iter().map(|w| w.cp_fmt()).collect()
        } else {
            // With separator
            let sep_str = SEP.to_string();
            self.0
                .into_iter()
                .map(|w| w.cp_fmt())
                .collect::<Vec<_>>()
                .join(&sep_str)
        }
    }
}

// Implement CPFormat for common types
macro_rules! impl_cp_format {
    ($($t:ty),* $(,)?) => {
        $(
            impl CPFormat for $t {
                fn cp_fmt(self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_cp_format!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, char, String, &str
);

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
    pub fn read_line<F, T>(&mut self, parser: F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        self.buf.clear();
        self.reader
            .read_line(&mut self.buf)
            .map_err(|err| format!("IO error: {}", err))?;
        parser(self.buf.trim())
    }

    /// Read multiple lines and parse each using the provided parser
    pub fn read_lines<F, T>(&mut self, n: usize, parser: F) -> Result<Vec<T>>
    where
        F: Fn(&str) -> Result<T>,
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
pub fn parse<T: FromStr>(s: &str) -> Result<T>
where
    T::Err: ToString,
{
    s.parse::<T>()
        .map_err(|err| format!("Failed to parse '{}': {}", s, err.to_string()))
}

/// Parse space-separated values from string
pub fn parse_vec<T: FromStr>(s: &str) -> Result<Vec<T>>
where
    T::Err: ToString,
{
    s.split_whitespace().map(|token| parse(token)).collect()
}

/// Parse space-separated values into fixed-size array
pub fn parse_array<T: FromStr + Debug, const N: usize>(s: &str) -> Result<[T; N]>
where
    T::Err: ToString,
{
    let vec: Vec<T> = parse_vec(s)?;
    vec.try_into()
        .map_err(|v: Vec<T>| format!("Expected {} elements, got {}", N, v.len()))
}

/// Parse string into vector of characters
pub fn parse_chars(s: &str) -> Result<Vec<char>> {
    Ok(s.chars().collect())
}

/// Parse binary string (0s and 1s) into vector of u8
pub fn parse_binary(s: &str) -> Result<Vec<u8>> {
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
    O: CPFormat,
{
    let mut input = CPInput::new(stdin().lock());
    let output = solution(&mut input);
    let mut stdout = stdout().lock();
    stdout.write_all(output.cp_fmt().as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
}

/// Solve multiple test cases problem
pub fn solve_n<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> O)
where
    O: CPFormat,
{
    let mut input = CPInput::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let n: usize = input.read_line(parse).unwrap();
    for _ in 0..n {
        let output = solution(&mut input);
        writer.write_all(output.cp_fmt().as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
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
