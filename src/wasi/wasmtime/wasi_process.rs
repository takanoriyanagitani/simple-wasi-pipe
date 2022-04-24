use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtx;

use crate::module::ModuleInfo;
use crate::process::Process;

use crate::wasi::wasmtime::ctx_builder::{new_static_ctx_builder_from_module_info, CtxBuilder};

pub fn new_pipes_from_modules<I>(i: I) -> Result<Vec<Box<dyn Process>>, String>
where
    I: Iterator<Item = ModuleInfo>,
{
    i.map(new_boxed_processor_from_module_info).collect()
}

fn new_boxed_processor_from_module_info(m: ModuleInfo) -> Result<Box<dyn Process>, String> {
    let b = new_static_ctx_builder_from_module_info(m)?; // impl CtxBuilder
    Ok(Box::new(CtxProcess { b }))
}

struct CtxProcess<B> {
    b: B,
}
impl<B> Process for CtxProcess<B>
where
    B: CtxBuilder,
{
    fn process(&mut self, i: &[u8], o: &mut Vec<u8>) -> Result<(), String> {
        let ctx: WasiCtx = self.b.build()?;
        let f = |cx: WasiCtx| {
            ctx_run(
                cx,
                self.b.as_engine(),
                self.b.as_module(),
                self.b.as_linker(),
            )
        };
        ctx_process(ctx, i, o, f)
    }
}

fn ctx_process<F>(mut ctx: WasiCtx, i: &[u8], o: &mut Vec<u8>, f: F) -> Result<(), String>
where
    F: Fn(WasiCtx) -> Result<(), String>,
{
    let internal: Vec<u8> = Vec::with_capacity(o.len());

    let stdin = ReadPipe::from(i);
    ctx.set_stdin(Box::new(stdin));

    let stdout = WritePipe::new(internal);
    ctx.set_stdout(Box::new(stdout.clone()));

    f(ctx)?;

    let mut done: Vec<u8> = stdout
        .try_into_inner()
        .map_err(|_| String::from("Unable to get buffer"))?;
    o.clear();
    o.append(&mut done);
    Ok(())
}

fn ctx_run(ctx: WasiCtx, e: &Engine, m: &Module, l: &Linker<WasiCtx>) -> Result<(), String> {
    let mut s = Store::new(e, ctx);
    let i = l
        .instantiate(&mut s, m)
        .map_err(|e| format!("Unable to get an instance: {}", e))?;
    let main = i
        .get_typed_func::<(), (), _>(&mut s, "_start")
        .map_err(|e| format!("No entry point: {}", e))?;
    main.call(&mut s, ())
        .map_err(|e| format!("Unable to call main: {}", e))
}
