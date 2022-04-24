use std::fs::File;
use std::path::Path;

use wasmtime::{Engine, Linker, Module};
use wasmtime_wasi::{sync::Dir, WasiCtx, WasiCtxBuilder};

use crate::module::{ModuleInfo, ModuleInfoConsumer};

pub trait CtxBuilder {
    fn build(&self) -> Result<WasiCtx, String>;
    fn as_engine(&self) -> &Engine;
    fn as_module(&self) -> &Module;
    fn as_linker(&self) -> &Linker<WasiCtx>;
}

pub fn new_static_ctx_builder_from_module_info(m: ModuleInfo) -> Result<impl CtxBuilder, String> {
    let b = BuilderSource::from(m);
    b.into_static_ctx_builder()
}

struct BuilderSource {
    module_path: String,
    env_pairs: Vec<(String, String)>,
    map_pairs: Vec<(String, String)>,
}
impl BuilderSource {
    fn empty() -> Self {
        Self {
            module_path: String::default(),
            env_pairs: vec![],
            map_pairs: vec![],
        }
    }

    fn into_static_ctx_builder(self) -> Result<impl CtxBuilder, String> {
        new_static_ctx_builder(self.module_path.as_str(), self.env_pairs, self.map_pairs)
    }
}
impl From<ModuleInfo> for BuilderSource {
    fn from(m: ModuleInfo) -> Self {
        let mut b = Self::empty();
        m.into_consumer(&mut b);
        b
    }
}
impl ModuleInfoConsumer for BuilderSource {
    fn consume_module_path(&mut self, m: String) {
        self.module_path = m;
    }
    fn consume_env(&mut self, e: Vec<(String, String)>) {
        self.env_pairs = e;
    }
    fn consume_map(&mut self, m: Vec<(String, String)>) {
        self.map_pairs = m;
    }
}

fn new_static_ctx_builder(
    module_path: &str,
    env_pairs: Vec<(String, String)>,
    dir_pairs: Vec<(String, String)>,
) -> Result<impl CtxBuilder, String> {
    StaticContextBuilder::new(module_path, env_pairs, dir_pairs)
}

trait EnvSource {
    fn env_pairs(&self) -> &[(String, String)];
}
trait DirSource {
    fn dir_pairs(&self) -> &[(String, String)];
}

struct StaticContextBuilder {
    e: Engine,
    m: Module,
    l: Linker<WasiCtx>,
    env_pairs: Vec<(String, String)>,
    dir_pairs: Vec<(String, String)>,
}
impl StaticContextBuilder {
    fn new(
        module_path: &str,
        env_pairs: Vec<(String, String)>,
        dir_pairs: Vec<(String, String)>,
    ) -> Result<Self, String> {
        let e = Engine::default();
        let m = Module::from_file(&e, module_path).map_err(|e| format!("Invalid module: {}", e))?;
        let mut l = Linker::new(&e);
        wasmtime_wasi::add_to_linker(&mut l, |x| x)
            .map_err(|e| format!("Unable to setup linker: {}", e))?;
        Ok(Self {
            e,
            m,
            l,
            env_pairs,
            dir_pairs,
        })
    }
}
impl CtxBuilder for StaticContextBuilder {
    fn build(&self) -> Result<WasiCtx, String> {
        let bldr = new_builder_from_sources(self, self)?;
        let host_stderr_used = bldr.inherit_stderr();
        Ok(host_stderr_used.build())
    }
    fn as_engine(&self) -> &Engine {
        &self.e
    }
    fn as_module(&self) -> &Module {
        &self.m
    }
    fn as_linker(&self) -> &Linker<WasiCtx> {
        &self.l
    }
}
impl EnvSource for StaticContextBuilder {
    fn env_pairs(&self) -> &[(String, String)] {
        &self.env_pairs
    }
}
impl DirSource for StaticContextBuilder {
    fn dir_pairs(&self) -> &[(String, String)] {
        &self.dir_pairs
    }
}

fn new_builder_from_sources<D, E>(d: &D, e: &E) -> Result<WasiCtxBuilder, String>
where
    D: DirSource,
    E: EnvSource,
{
    let ex = EnvCtxBuilder::new().consume_env_source(e)?;
    let dx = DirCtxBuilder::from(ex).with_dir_source(d)?;
    Ok(WasiCtxBuilder::from(dx))
}

pub struct DirCtxBuilder {
    b: WasiCtxBuilder,
}
impl DirCtxBuilder {
    fn new() -> Self {
        let b = WasiCtxBuilder::new();
        Self { b }
    }

    pub fn map_dir(self, host: &Path, guest: &Path) -> Result<Self, String> {
        let f = File::open(host).map_err(|e| format!("Unable to open host dir: {}", e))?;
        let d = Dir::from_std_file(f);
        let neo = self
            .b
            .preopened_dir(d, guest)
            .map_err(|e| format!("Unable to map dir: {}", e))?;
        Ok(Self::from(neo))
    }

    fn with_dir_source<S>(self, s: &S) -> Result<Self, String>
    where
        S: DirSource,
    {
        let dir_pairs: &[(String, String)] = s.dir_pairs();
        dir_pairs.iter().try_fold(self, |ctx, (host, guest)| {
            let hp = Path::new(host.as_str());
            let gp = Path::new(guest.as_str());
            ctx.map_dir(hp, gp)
        })
    }
}
impl From<WasiCtxBuilder> for DirCtxBuilder {
    fn from(b: WasiCtxBuilder) -> Self {
        Self { b }
    }
}
impl From<DirCtxBuilder> for WasiCtxBuilder {
    fn from(d: DirCtxBuilder) -> Self {
        d.b
    }
}
impl TryFrom<&[(&str, &str)]> for DirCtxBuilder {
    type Error = String;
    fn try_from(pairs: &[(&str, &str)]) -> Result<Self, Self::Error> {
        pairs.iter().try_fold(Self::new(), |ctx, pair| {
            let host = Path::new(pair.0);
            let guest = Path::new(pair.1);
            ctx.map_dir(host, guest)
        })
    }
}
impl From<EnvCtxBuilder> for DirCtxBuilder {
    fn from(e: EnvCtxBuilder) -> Self {
        let b: WasiCtxBuilder = e.into();
        Self::from(b)
    }
}

pub struct EnvCtxBuilder {
    b: WasiCtxBuilder,
}
impl EnvCtxBuilder {
    fn new() -> Self {
        let b = WasiCtxBuilder::new();
        Self { b }
    }

    pub fn set_env(self, key: &str, val: &str) -> Result<Self, String> {
        let neo = self
            .b
            .env(key, val)
            .map_err(|e| format!("Unable to set env: {}", e))?;
        Ok(Self::from(neo))
    }

    fn consume_env_source<S>(self, s: &S) -> Result<Self, String>
    where
        S: EnvSource,
    {
        let env_pairs: &[(String, String)] = s.env_pairs();
        env_pairs.iter().try_fold(self, |ctx, (key, val)| {
            ctx.set_env(key.as_str(), val.as_str())
        })
    }
}
impl From<WasiCtxBuilder> for EnvCtxBuilder {
    fn from(b: WasiCtxBuilder) -> Self {
        Self { b }
    }
}
impl From<EnvCtxBuilder> for WasiCtxBuilder {
    fn from(e: EnvCtxBuilder) -> Self {
        e.b
    }
}
impl TryFrom<&[(&str, &str)]> for EnvCtxBuilder {
    type Error = String;
    fn try_from(pairs: &[(&str, &str)]) -> Result<Self, Self::Error> {
        pairs
            .iter()
            .try_fold(Self::new(), |ctx, (key, val)| ctx.set_env(key, val))
    }
}
impl From<DirCtxBuilder> for EnvCtxBuilder {
    fn from(d: DirCtxBuilder) -> Self {
        let b: WasiCtxBuilder = d.into();
        Self::from(b)
    }
}
