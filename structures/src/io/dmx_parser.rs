extern crate find_folder;

use std::io::prelude::*;
use std::fs::File;

use std::error::Error;

use std::collections::HashMap;

use io::config::{get_config_path, Config};

use logic::ChannelGroup;
use logic::channel::DmxAddress;
use logic::channel::DmxValue;

use logic::Stage;
use logic::Fixture;

use logic::channel_group::RGB;
use logic::channel_group::RGBA;
use logic::channel_group::Single;
use logic::channel_group::Moving2D;

use FIXTURE_DEF;

pub struct Parser {
    stage: Stage
}

impl Parser {
    pub fn new(stage: Stage) -> Parser {
        Parser {
            stage: stage
        }
    }

    pub fn parse(mut self) -> Stage {
        self.read_file();
        self.stage
    }

    fn parse_scene(&mut self, block: Vec<char>, fixtures: &HashMap<String, Vec<String>>) {
        block_to_commands(block).into_iter().map(|command| {
            let mut name_def_open = false;
            let mut name = Vec::new();
            let mut fixture_type_open = false;
            let mut fixture_type = Vec::new();
            let mut target_channel_char = Vec::new();
            command.chars().map(|char| {
                if char == as_char("-") && !fixture_type_open {
                    fixture_type_open = true;
                    return
                } else if char == as_char(">") && !name_def_open { return }
                else if char == as_char("(") && !name_def_open { name_def_open = true; return }
                if name_def_open {
                    name.push(char);
                } else if fixture_type_open {
                    fixture_type.push(char);
                } else {
                    target_channel_char.push(char);
                }
            }).collect::<Vec<_>>();
            let target_channel;
            match vec_as_str(target_channel_char.clone()).parse::<DmxAddress>() {
                Ok(v) => target_channel = v,
                Err(_) => {exit!(10, "Invalid channel ID: {:?}", target_channel_char);}
            }
            strip_parentheses(&mut fixture_type);
            strip_parentheses(&mut name);
            name.pop();
            let fixture_type_str = vec_as_str(fixture_type);
            let name_str = vec_as_str(name);
            if !fixtures.contains_key(&fixture_type_str) { exit!(10, "Fixture '{}' is not defined.", fixture_type_str); }
            //trace!("{:?} | {:?} | {:?}", target_channel, fixture_type_str, name_str);
            self.parse_fixture(fixtures.get(&fixture_type_str).unwrap().clone(), target_channel as u16, name_str);
        }).collect::<Vec<_>>();
    }

    fn parse_fixture(&mut self, commands: Vec<String>, start_channel: DmxAddress, name: String) {
        let mut channel_groups = Vec::new();

        commands.into_iter().map(|command| {
            let mut command_type = Vec::new();
            let mut command_chars = Vec::new();
            let mut command_open = 0;
            command.chars().map(|char| {
                if command_open > 0 {
                    command_chars.push(char);
                } else {
                    command_type.push(char);
                }
                if char == as_char("(") {
                    command_open += 1;
                } else if char == as_char(")") {
                    if command_open > 1 { command_open -= 1 }
                    else if command_open == 1 {
                        command_open = 0;
                        command_chars.pop();
                        strip_parentheses(&mut command_chars);
                        let command_args = char_vec_to_string_vec(command_chars.clone());

                        command_type.pop();
                        let command_type_str = vec_as_str(command_type.clone());
                        // let command_content_str = vec_as_str(command_chars.clone());
                        //trace!("Got new command of type {} with content {}", command_type_str, command_content_str);

                        if command_type_str == "preheat".to_string() {
                            //trace!("PREHEAT WOHOOOO");
                            let cg = channel_groups.pop().expect(&format!("No channel group defined for preheat. ({})", command));

                            channel_groups.push(match cg {
                                ChannelGroup::Single(group) => {
                                    group.channel1.lock().expect("Failed to lock Arc!").max_preheat_value = command_args[0].parse::<DmxValue>().unwrap();
                                    ChannelGroup::Single(group)
                                },
                                _ => {
                                    exit!(10, "Preheat is not available for fixture type {:?}", cg);
                                }
                            });
                        } else {
                            match command_type_str.trim() {
                                "rgb" => {
                                    channel_groups.push(
                                        ChannelGroup::RGB(RGB::new(
                                            self.stage.get_channel_object(start_channel + (command_args[0].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[1].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[2].parse::<DmxAddress>().unwrap()))
                                        ))
                                    );
                                },
                                "rgba" => {
                                    channel_groups.push(
                                        ChannelGroup::RGBA(RGBA::new(
                                            self.stage.get_channel_object(start_channel + (command_args[0].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[1].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[2].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[3].parse::<DmxAddress>().unwrap()))
                                        ))
                                    );
                                },
                                "single" => {
                                    channel_groups.push(
                                        ChannelGroup::Single(Single::new(
                                            self.stage.get_channel_object(start_channel + (command_args[0].parse::<DmxAddress>().unwrap()))
                                        ))
                                    );
                                },
                                "moving_2d" => {
                                    channel_groups.push(
                                        ChannelGroup::Moving2D(Moving2D::new(
                                            self.stage.get_channel_object(start_channel + (command_args[0].parse::<DmxAddress>().unwrap())),
                                            self.stage.get_channel_object(start_channel + (command_args[1].parse::<DmxAddress>().unwrap()))
                                        ))
                                    );
                                },
                                _ => {}
                            }
                        }
                        command_type.clear();
                        command_chars.clear();
                    } else if command_open == 0 { exit!(10, "Unclosed delimiter. Figure out where it is."); }
                }
            }).collect::<Vec<_>>();
            //trace!("COMMAND DONE");
        }).collect::<Vec<_>>();

        self.stage.add_fixture(Fixture::new(name, channel_groups));
    }

    fn read_file(&mut self) {
        let fixture_tag = "Fixture".to_string();
        let stage_tag = "Stage".to_string();

        let path = get_config_path(Config::Server, &self.stage.name).join(FIXTURE_DEF);
        let mut f = match File::open(path) {
            Ok(f) => f, Err(e) => {
                exit!(9, "Unable to read fixture definitions: {}", e.description());
            }
        };
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let mut blocks_open = 0;
        let mut fixtures = HashMap::new();
        // let mut stages = Vec::new();
        let mut block = Vec::new();
        let mut block_type = Vec::new();
        //Get all blocks aka {}
        for (index, char) in s.char_indices() {
            if blocks_open > 0 {
                block.push(char);
            } else {
                block_type.push(char);
            }
            if char == as_char("{") {
                blocks_open += 1;
            } else if char == as_char("}") {
                if blocks_open > 1 { blocks_open -= 1 }
                else if blocks_open == 1 {
                    blocks_open = 0;
                    block.pop();
                    block_type.pop();
                    let block_type_str = vec_as_str(block_type);
                    if block_type_str == fixture_tag {
                        let mut commands = block_to_commands(block);
                        let name = (&(commands[0].clone())[6..(commands[0].len()-2)]).to_string();
                        commands.remove(0);
                        fixtures.insert(name, commands);
                    } else if block_type_str == stage_tag {
                        self.parse_scene(block, &fixtures);
                    } else { exit!(10, "Unknown tag {:?}", block_type_str); }
                    block_type = Vec::new();
                    block = Vec::new();
                } else if blocks_open == 0 { exit!(10, "Syntax error: Unexpected closing bracket @ {}", index); }
            }
        }
        if blocks_open > 0 { exit!(10, "Syntax error: Unclosed block."); }

        // for block in fixtures.iter() {trace!("{:?}", block);}
        // for block in stages.iter() {trace!("{:?}", block);}
    }
}

fn char_vec_to_string_vec(mut v1: Vec<char>) -> Vec<String> {
    v1.push(as_char(","));
    v1.into_iter().fold((Vec::new(), Vec::new()), |acc, char| {
        let mut a = acc;
        if char == as_char(",") {
            a.0.push(vec_as_str(a.1));
            (a.0, Vec::new())
        } else {
            a.1.push(char);
            a
        }
    }).0
}


fn strip_unneccessary_symbols(vector: &mut Vec<char>) {
    vector.retain(|&x| !(x == "\n".to_string().remove(0) || x == " ".to_string().remove(0)));
}

fn strip_parentheses(vector: &mut Vec<char>) {
    vector.retain(|&x| !(x == "\"".to_string().remove(0)));
}

fn vec_as_str(mut vector: Vec<char>) -> String {
    strip_unneccessary_symbols(&mut vector);
    vector.iter().fold(String::new(), |acc, &x| {
        format!("{}{}", acc, x)
    })
}

fn block_to_commands(mut block: Vec<char>) -> Vec<String> {
    strip_unneccessary_symbols(&mut block);
    block.iter().fold((Vec::new(), String::new()), |acc, &x| {
        let mut commands = acc.0;
        let command = acc.1;
        if x == as_char(";") {
            commands.push(command);
            return (commands, String::new());
        } else {
            return (commands, format!("{}{}", command, x));
        }
    }).0
}

fn as_char(string: &'static str) -> char {
    string.to_string().remove(0)
}
