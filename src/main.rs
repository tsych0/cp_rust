use cf_rust::cpio;

// @code begin
use cpio::solve_n;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = input.read_line(cpio::parse).unwrap();
    n > 2 && n % 2 == 0
}
// @code end
