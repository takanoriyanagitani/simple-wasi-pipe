#[derive(serde::Deserialize)]
struct Env {
    key: String,
    val: String,
}
impl Env {
    fn into_pair(self) -> (String, String) {
        (self.key, self.val)
    }
}

#[derive(serde::Deserialize)]
struct Map {
    host: String,
    guest: String,
}
impl Map {
    fn into_pair(self) -> (String, String) {
        (self.host, self.guest)
    }
}

pub trait ModuleInfoConsumer {
    fn consume_module_path(&mut self, m: String);
    fn consume_env(&mut self, e: Vec<(String, String)>);
    fn consume_map(&mut self, m: Vec<(String, String)>);
}

#[derive(serde::Deserialize)]
pub struct ModuleInfo {
    module_path: String,
    env: Vec<Env>,
    map: Vec<Map>,
}
impl ModuleInfo {
    pub fn into_consumer<C>(self, c: &mut C)
    where
        C: ModuleInfoConsumer,
    {
        c.consume_module_path(self.module_path);
        c.consume_env(self.env.into_iter().map(|e| e.into_pair()).collect());
        c.consume_map(self.map.into_iter().map(|m| m.into_pair()).collect());
    }
}

pub trait ModuleInfoSource {
    fn get(&mut self) -> Result<ModuleInfo, String>;
}

pub fn new_json_module_info_iter<I>(i: I) -> Result<Vec<ModuleInfo>, String>
where
    I: Iterator<Item = String>,
{
    i.map(JsonInfoSource::from).map(|mut s| s.get()).collect()
}

struct JsonInfoSource {
    json: String,
}
impl From<String> for JsonInfoSource {
    fn from(json: String) -> Self {
        Self { json }
    }
}
impl ModuleInfoSource for JsonInfoSource {
    fn get(&mut self) -> Result<ModuleInfo, String> {
        serde_json::from_str(self.json.as_str()).map_err(|e| format!("Invalid module info: {}", e))
    }
}
