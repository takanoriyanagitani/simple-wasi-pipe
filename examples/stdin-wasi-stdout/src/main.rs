use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use simple_wasi_pipe::Process;
use simple_wasi_pipe::{stdin2pipes2stdout, strings2pipes};

fn file2strings(f: File) -> Result<Vec<String>, io::Error> {
    let b = BufReader::new(f);
    let lines = b.lines();
    lines.collect()
}

fn file2pipes(f: File) -> Result<Vec<Box<dyn Process>>, String> {
    let strings: Vec<String> =
        file2strings(f).map_err(|e| format!("Unable to get module info: {}", e))?;
    strings2pipes(strings.into_iter())
}

fn stdin2procs2stdout(f: File) -> Result<(), String> {
    let pipes = file2pipes(f)?;
    let mut ibuf: Vec<u8> = Vec::with_capacity(1048576);
    let mut obuf: Vec<u8> = Vec::with_capacity(1048576);
    stdin2pipes2stdout(&mut pipes.into_iter(), &mut ibuf, &mut obuf)
}

fn sub() -> Result<(), String> {
    let module_conf: String =
        env::var("ENV_MODULE_CONF").unwrap_or_else(|_| String::from("./sample.conf.jsonl"));
    let f = File::open(module_conf).map_err(|e| format!("Unable to open module conf: {}", e))?;
    stdin2procs2stdout(f)
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
