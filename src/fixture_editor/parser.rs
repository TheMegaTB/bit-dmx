use rustc_serialize::json;
use structures::DmxAddress;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub enum ChannelGroup {
    Single(DmxAddress),
    RGB(DmxAddress, DmxAddress, DmxAddress),
    RGBA((DmxAddress, DmxAddress, DmxAddress, DmxAddress)),
    Moving2D((DmxAddress, DmxAddress))
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct FixtureTemplate {
    pub name: String,
    pub channel_groups: Vec<ChannelGroup>,
}

impl FixtureTemplate {
    fn new(n: String, t: Vec<ChannelGroup>) -> FixtureTemplate {
        FixtureTemplate {
            name: n,
            channel_groups: t,
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Fixture {
    pub channel: i32,
    pub template_name: String,
    pub name: String,
}

impl Fixture {
    fn new(c: i32, tN: String, n: String) -> Fixture {
        Fixture {
            channel: c,
            template_name: tN,
            name: n,
        }
    }
    fn new_empty() -> Fixture {
        Fixture {
            channel: 0,
            template_name: "Empty".to_string(),
            name: "Empty".to_string(),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Stage {
    pub name: String,
    pub fixture: Vec<Fixture>,
}

impl Stage {
    fn new(n: String, c: Vec<Fixture>) -> Stage {
        Stage {
            name: n,
            fixture: c,
        }
    }
    fn new_empty() -> Stage {
        Stage {
            name: "Untitled".to_string(),
            fixture: Vec::new(),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Config {
    pub fixture_templates: Vec<FixtureTemplate>,
    pub stage: Stage,
}

impl Config {
    fn new(f: Vec<FixtureTemplate>, s: Stage) -> Config {
        Config {
            fixture_templates: f,
            stage: s,
        }
    }
    pub fn new_empty() -> Config {
        Config {
            fixture_templates: Vec::new(),
            stage: Stage::new_empty(),
        }
    }
}
pub fn parse_file() -> Config {
    let mut fixtures: Vec<FixtureTemplate> = vec![];

    fixtures.push(FixtureTemplate::new("Test".to_string(), vec!(ChannelGroup::RGB(0, 1, 2))));
    fixtures.push(FixtureTemplate::new("Test2".to_string(), vec!(ChannelGroup::Single(0))));

    Config::new(fixtures, Stage::new_empty())
}

pub fn decode_file(content: String) -> Config {
    let decoded: Config = json::decode(&content).unwrap();
    decoded
}

pub fn encode_file(file: Config) -> String {
    let encoded = json::encode(&file).unwrap();
    encoded
}
