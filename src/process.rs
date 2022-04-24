use std::io::{Read, Write};

pub trait Process {
    fn process(&mut self, i: &[u8], o: &mut Vec<u8>) -> Result<(), String>;
}

pub trait ProcessBuilder<P>
where
    P: Process,
{
    fn build(self) -> Result<P, String>;
}

fn mem2pipes2mem<I>(i: &mut I, ibuf: &mut Vec<u8>, obuf: &mut Vec<u8>) -> Result<(), String>
where
    I: Iterator<Item = Box<dyn Process>>,
{
    for mut p in i {
        let si: &[u8] = ibuf;
        obuf.clear();
        p.process(si, obuf)?;
        ibuf.clear();
        ibuf.append(obuf);
    }
    obuf.clear();
    obuf.append(ibuf);
    Ok(())
}

pub fn read2pipes2write<R, I, W>(
    r: &mut R,
    i: &mut I,
    w: &mut W,
    ibuf: &mut Vec<u8>,
    obuf: &mut Vec<u8>,
) -> Result<(), String>
where
    R: Read,
    W: Write,
    I: Iterator<Item = Box<dyn Process>>,
{
    ibuf.clear();
    r.read_to_end(ibuf)
        .map_err(|e| format!("Unable to read input: {}", e))?;
    mem2pipes2mem(i, ibuf, obuf)?;
    w.write_all(obuf)
        .map_err(|e| format!("Unable to write data: {}", e))?;
    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}
