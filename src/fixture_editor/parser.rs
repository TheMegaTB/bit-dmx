use rustc_serialize::json;
use structures::logic::channel::DmxAddress;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub enum ChannelGroup {
    Single(DmxAddress),
    RGB(DmxAddress, DmxAddress, DmxAddress),
    RGBA(DmxAddress, DmxAddress, DmxAddress, DmxAddress),
    Moving2D(DmxAddress, DmxAddress)
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FixtureTemplate {
    pub name: String,
    pub channel_groups: Vec<ChannelGroup>,
}

impl FixtureTemplate {
    pub fn new_empty() -> FixtureTemplate {
        FixtureTemplate {
            name: "Untitled".to_string(),
            channel_groups: Vec::new(),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Fixture {
    pub channel: i32,
    pub template_id: i32,
    pub name: String,
}

impl Fixture {
    /*pub fn new(c: i32, t_n: String, n: String) -> Fixture {
        Fixture {
            channel: c,
            template_name: t_n,
            name: n,
        }
    }*/
    pub fn new_empty(id: i32) -> Fixture {
        Fixture {
            channel: 0,
            template_id: id,
            name: "Empty".to_string(),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Stage {
    pub name: String,
    pub fixtures: Vec<Fixture>,
}

impl Stage {
    fn new_empty() -> Stage {
        Stage {
            name: "Untitled".to_string(),
            fixtures: Vec::new(),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Config {
    pub fixture_templates: Vec<FixtureTemplate>,
    pub stage: Stage,
}

impl Config {
    pub fn new_empty() -> Config {
        Config {
            fixture_templates: Vec::new(),
            stage: Stage::new_empty(),
        }
    }
}

pub fn decode_file(content: String) -> Option<Config> {
    let result = json::decode(&content);
    let decoded: Config;

    match result {
        Ok(config) => decoded = config,
        _ => return None
    }
    Some(decoded)
}

pub fn encode_file(file: Config) -> String {
    let encoded = json::encode(&file).unwrap();
    encoded
}
