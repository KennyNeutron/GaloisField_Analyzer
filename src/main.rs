mod gfpoly;

use gfpoly::GFPoly;
use std::io::{self, BufRead, Write};

fn process_io<R: BufRead, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap()?.trim().parse().unwrap();

    for i in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let mut parts = line.split_whitespace();
            let n: usize = parts.next().unwrap().parse().unwrap();
            let modulo: u16 = parts.next().unwrap().parse().unwrap();
            let coef_vals: Vec<u8> = parts.take(n).map(|x| x.parse().unwrap()).collect();

            let poly = GFPoly::with_coefs(coef_vals, modulo);
            writeln!(writer, "Message #{}: {}", i + 1, poly)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    process_io(stdin.lock(), stdout.lock())
}
