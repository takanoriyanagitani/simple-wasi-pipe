use std::env;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn read2filter2writer<R, F, W>(r: &mut R, f: F, w: &mut W) -> Result<(), String>
where
    R: BufRead,
    W: Write,
    F: Fn(&str) -> bool,
{
    let lines = r.lines();
    let err_skip = lines.flat_map(|rslt| rslt.ok()); // Iterator<String>
    let mut filtered = err_skip.filter(|s| f(s)); // Iterator<String>
    filtered.try_for_each(|s| {
        writeln!(w, "{}", s).map_err(|e| format!("Unable to write: {}", e))?;
        Ok(())
    })
}

fn stdin2filter2stdout<F>(f: F) -> Result<(), String>
where
    F: Fn(&str) -> bool,
{
    let i = stdin();
    let il = i.lock();
    let mut ib = BufReader::new(il);

    let o = stdout();
    let mut ol = o.lock();
    let mut ob = BufWriter::new(ol.by_ref());

    read2filter2writer(&mut ib, f, &mut ob)?;
    ob.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    drop(ob);
    ol.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn sub() -> Result<(), String> {
    let start_str: String = env::var("ENV_START").unwrap_or_default();
    let f = |line: &str| line.starts_with(start_str.as_str());
    stdin2filter2stdout(f)
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
