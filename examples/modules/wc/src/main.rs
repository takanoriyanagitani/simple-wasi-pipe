use std::io;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

fn count_lines<I, E>(mut i: I) -> Result<u64, E>
where
    I: Iterator<Item = Result<Vec<u8>, E>>,
{
    i.try_fold(0, |cnt, rslt| rslt.map(|_| cnt + 1))
}

fn reader2line_count<R>(r: R) -> Result<u64, io::Error>
where
    R: BufRead,
{
    let splited = r.split(b'\n');
    count_lines(splited)
}

fn count2wtr<W>(cnt: u64, w: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    writeln!(w, "{}", cnt)
}

fn stdin2count2stdout() -> Result<(), io::Error> {
    let i = stdin();
    let il = i.lock();
    let ib = BufReader::new(il);
    let line_cnt: u64 = reader2line_count(ib)?;

    let o = stdout();
    let mut ol = o.lock();
    count2wtr(line_cnt, &mut ol)
}

fn sub() -> Result<(), io::Error> {
    stdin2count2stdout()
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
