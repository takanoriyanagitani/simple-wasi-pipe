use std::env;
use std::fs::{read_dir, DirEntry};
use std::io::{stdout, BufWriter, Write};

fn dir2dirent<F>(dir: &str, mut f: F) -> Result<(), String>
where
    F: FnMut(&DirEntry) -> Result<(), String>,
{
    let r = read_dir(dir).map_err(|e| format!("Unable to read dir: {}", e))?;
    let mut items = r.flat_map(|rslt| rslt.ok()); // Iterator<DirEntry>
    items.try_for_each(|dirent| f(&dirent))
}

fn dirent2wtr<W>(d: &DirEntry, w: &mut W) -> Result<(), String>
where
    W: Write,
{
    let p = d.path(); // PathBuf
    let s: &str = p
        .to_str()
        .ok_or_else(|| String::from("Unable to convert to str"))?;
    writeln!(w, "{}", s).map_err(|e| format!("Unable to write path: {}", e))?;
    Ok(())
}

fn dir2wtr<W>(dir: &str, w: &mut W) -> Result<(), String>
where
    W: Write,
{
    let f = |dirent: &DirEntry| dirent2wtr(dirent, w);
    dir2dirent(dir, f)?;
    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn dir2stdout(dir: &str) -> Result<(), String> {
    let o = stdout();
    let mut ol = o.lock();
    let mut ob = BufWriter::new(ol.by_ref());
    dir2wtr(dir, &mut ob)?;
    drop(ob);
    ol.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn sub() -> Result<(), String> {
    let dir: String = env::var("ENV_DIR").unwrap_or_else(|_| String::from("."));
    dir2stdout(dir.as_str())
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
