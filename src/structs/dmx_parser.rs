extern crate find_folder;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;
use std::sync::mpsc;

use ChannelGroup;
use DmxChannel;
use RGB;
use RGBA;
use Single;

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

fn parse_scene(block: Vec<char>, fixtures: &HashMap<String, Vec<String>>) {
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
        let target_channel: u32;
        match target_channel_char[0].to_digit(10) {
            Some(v) => target_channel = v,
            None => panic!("Invalid channel ID: {:?}", target_channel_char)
        }
        strip_parentheses(&mut fixture_type);
        strip_parentheses(&mut name);
        name.pop();
        let fixture_type_str = vec_as_str(fixture_type);
        let name_str = vec_as_str(name);
        if !fixtures.contains_key(&fixture_type_str) { panic!("Fixture '{}' is not defined.", fixture_type_str) }
        trace!("{:?} | {:?} | {:?}", target_channel, fixture_type_str, name_str);
        parse_fixture(fixtures.get(&fixture_type_str).unwrap().clone(), target_channel as u16);
    }).collect::<Vec<_>>();
}

fn parse_fixture(commands: Vec<String>, start_channel: DmxChannel) {
    let mut channel_groups: Vec<ChannelGroup> = Vec::new();

    commands.into_iter().map(|command| {
        let mut command_type = Vec::new();
        let mut command_content = Vec::new();
        let mut command_open = 0;
        let mut name = "";
        command.chars().map(|char| {
            if command_open > 0 {
                command_content.push(char);
            } else {
                command_type.push(char);
            }
            if char == as_char("(") {
                command_open += 1;
            } else if char == as_char(")") {
                if command_open > 1 { command_open -= 1 }
                else if command_open == 1 {
                    command_open = 0;
                    command_content.pop();
                    strip_parentheses(&mut command_content);
                    command_type.pop();
                    let command_type_str = vec_as_str(command_type.clone());
                    let command_content_str = vec_as_str(command_content.clone());
                    trace!("Got new command of type {} with content {}", command_type_str, command_content_str);
                    //TODO: Match the commands.
                    if command_type_str == "preheat".to_string() {
                        trace!("PREHEAT WOHOOOO");
                        let cg = channel_groups.pop().expect(&format!("No channel group defined for preheat. ({})", command));
                        //enable preheat
                        channel_groups.push(cg);
                    } else {
                        match command_type_str.trim() {
                            "rgb" => {
                                // channel_groups.push(ChannelGroup::RGB(
                                //     RGB::new();
                                // ));
                            },
                            "rgba" => {

                            },
                            "single" => {
                                channel_groups.push(ChannelGroup::Single(
                                    Single::new(start_channel + (command_content[0].to_digit(10).unwrap() as u16), mpsc::channel().0) //TODO: Insert real channel
                                ));
                            },
                            _ => {}
                        }
                    }
                    command_type.clear();
                    command_content.clear();
                } else if command_open == 0 { panic!("Unclosed delimiter. Figure out where it is.") }
            }
        }).collect::<Vec<_>>();
        println!("COMMAND DONE");
    }).collect::<Vec<_>>();
}

pub fn read_file() {
    let fixture_tag = "Fixture".to_string();
    let stage_tag = "Stage".to_string();

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets").unwrap();
    let path = assets.join("fixtures.dmx");
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    //println!("{}", s);

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
                    parse_scene(block, &fixtures);
                } else { panic!("Unknown tag {:?}", block_type_str) }
                block_type = Vec::new();
                block = Vec::new();
            } else if blocks_open == 0 { panic!("Syntax error: Unexpected closing bracket @ {}", index) }
        }
    }
    if blocks_open > 0 { panic!("Syntax error: Unclosed block.") }

    // for block in fixtures.iter() {trace!("{:?}", block);}
    // for block in stages.iter() {trace!("{:?}", block);}
}
