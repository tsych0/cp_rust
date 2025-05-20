use std::{
    fmt::Display,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Result, StdinLock, Write},
    str::FromStr,
};

pub enum ListOf<T> {
    WordsOf(Vec<T>),
    LinesOf(Vec<T>),
}

pub struct CPInput<R: Read> {
    reader: BufReader<R>,
    buf: String,
}

pub trait CPOutput {
    fn cp_fmt(&self) -> Vec<u8>;
}

// Specific implementation for bool
impl CPOutput for bool {
    fn cp_fmt(&self) -> Vec<u8> {
        if *self { "YES" } else { "NO" }.as_bytes().to_vec()
    }
}

impl<T> Display for ListOf<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ListOf::WordsOf(ws) => ws
                .into_iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            ListOf::LinesOf(ls) => ls
                .into_iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        };

        write!(f, "{}", res)
    }
}

impl<T> CPOutput for ListOf<T>
where
    T: Display,
{
    fn cp_fmt(&self) -> Vec<u8> {
        let res = match self {
            ListOf::WordsOf(ws) => ws
                .into_iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            ListOf::LinesOf(ls) => ls
                .into_iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        };

        res.as_bytes().to_vec()
    }
}

// Macro to implement CPDisplay for types that should delegate to Display
macro_rules! impl_cp_output {
                ($($t:ty),*) => {
                    $(
                        impl CPOutput for $t {
                            fn cp_fmt(&self) -> Vec<u8>  {
                                format!("{self}").as_bytes().to_vec()
                            }
                        }
                    )*
                };
            }

// Implement for common types
impl_cp_output!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, char, String
);

impl<R> CPInput<R>
where
    R: Read,
{
    pub fn new(input: R) -> Self {
        CPInput {
            reader: BufReader::new(input),
            buf: String::new(),
        }
    }

    pub fn read_line<F, T>(&mut self, parser: F) -> Result<T>
    where
        F: Fn(&str) -> T,
    {
        self.buf.clear();
        self.reader.read_line(&mut self.buf)?;
        Ok(parser(self.buf.trim()))
    }

    pub fn read_lines<F, T>(&mut self, n: usize, parser: F) -> Result<Vec<T>>
    where
        F: Fn(&str) -> T,
    {
        (1..=n)
            .map(|_| {
                self.buf.clear();
                self.reader.read_line(&mut self.buf)?;
                Ok(parser(&self.buf.trim()))
            })
            .collect()
    }
}

pub fn parse<T: FromStr>(s: &str) -> T {
    match s.parse() {
        Ok(v) => v,
        Err(_) => panic!("Error while parsing words"),
    }
}

pub fn parse_vec<T: FromStr>(s: &str) -> Vec<T> {
    s.split(' ')
        .map(|s| match s.parse() {
            Ok(v) => v,
            Err(_) => panic!("Error while parsing words"),
        })
        .collect()
}

pub fn parse_binary(s: &str) -> Vec<u8> {
    s.chars().map(|s| if s == '0' { 0 } else { 1 }).collect()
}

pub fn solve<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> O)
where
    O: CPOutput,
{
    let mut io = CPInput::new(stdin().lock());
    let output = solution(&mut io);
    stdout().lock().write_all(&output.cp_fmt()).unwrap();
}

pub fn solve_n<O>(solution: fn(&mut CPInput<StdinLock<'static>>) -> O)
where
    O: CPOutput,
{
    let mut input = CPInput::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let n = input.read_line(parse).unwrap();
    for _ in 0..n {
        let output = solution(&mut input);
        writer.write_all(&output.cp_fmt()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}
