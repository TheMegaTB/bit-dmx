extern crate find_folder;

use std::io::prelude::*;
use std::fs::File;

fn strip_unneccessary_symbols(vector: &mut Vec<char>) {
    vector.retain(|&x| !(x == "\n".to_string().remove(0) || x == " ".to_string().remove(0)));
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
        if x == ";".to_string().remove(0) {
            commands.push(command);
            return (commands, String::new());
        } else {
            return (commands, format!("{}{}", command, x));
        }
    }).0
}

pub fn read_file() {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets").unwrap();
    let path = assets.join("fixtures.dmx");
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    //println!("{}", s);

    let mut blocks_open = 0;
    let mut fixtures = Vec::new();
    let mut stages = Vec::new();
    let mut block = Vec::new();
    let mut block_type = Vec::new();
    //Get all blocks aka {}
    for (index, char) in s.char_indices() {
        if blocks_open > 0 {
            block.push(char);
        } else {
            block_type.push(char);
        }
        if char == "{".to_string().remove(0) {
            blocks_open += 1;
        }
        else if char == "}".to_string().remove(0) {
            if blocks_open > 1 { blocks_open -= 1 }
            else if blocks_open == 1 {
                blocks_open = 0;
                block.pop();
                block_type.pop();
                let block_type_str = vec_as_str(block_type);
                if block_type_str == "Fixture".to_string() {
                    fixtures.push(block);
                } else if block_type_str == "Stage".to_string() {
                    stages.push(block);
                } else { panic!("Unknown tag {:?}", block_type_str) }
                block_type = Vec::new();
                block = Vec::new();
            } else if blocks_open == 0 { panic!("Syntax error: Unexpected closing bracket @ {}", index) }
        }
    }
    if blocks_open > 0 { panic!("Syntax error: Unclosed block.") }

    //Strip all unneccesary symbols and restructure into commands instead of characters
    let fixtures = fixtures.into_iter().map(|block| {
        block_to_commands(block)
    }).collect::<Vec<_>>();

    let stages = stages.into_iter().map(|block| {
        block_to_commands(block)
    }).collect::<Vec<_>>();

    for block in fixtures.iter() {trace!("{:?}", block);}
    for block in stages.iter() {trace!("{:?}", block);}
}
