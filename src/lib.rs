use std::io::{stdin, stdout, BufReader, BufWriter, Write};

mod module;
mod process;
mod wasi;

pub use process::Process;

use crate::wasi::wasmtime::wasi_process::new_pipes_from_modules;
use module::{new_json_module_info_iter, ModuleInfo};
use process::read2pipes2write;

fn modules2pipes<I>(m: I) -> Result<Vec<Box<dyn Process>>, String>
where
    I: Iterator<Item = ModuleInfo>,
{
    new_pipes_from_modules(m)
}

fn strings2modules<I>(s: I) -> Result<Vec<ModuleInfo>, String>
where
    I: Iterator<Item = String>,
{
    new_json_module_info_iter(s)
}

pub fn strings2pipes<I>(s: I) -> Result<Vec<Box<dyn Process>>, String>
where
    I: Iterator<Item = String>,
{
    let modules = strings2modules(s)?;
    modules2pipes(modules.into_iter())
}

pub fn stdin2pipes2stdout<I>(
    pipes: &mut I,
    ibuf: &mut Vec<u8>,
    obuf: &mut Vec<u8>,
) -> Result<(), String>
where
    I: Iterator<Item = Box<dyn Process>>,
{
    let i = stdin();
    let il = i.lock();
    let mut ib = BufReader::new(il);
    let o = stdout();
    let mut ol = o.lock();
    let mut ob = BufWriter::new(ol.by_ref());

    read2pipes2write(&mut ib, pipes, &mut ob, ibuf, obuf)?;
    drop(ob);
    ol.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
