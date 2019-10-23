/*
 *  This program manages usernames and emails for git.
 *  Copyright (C) 2019  Cameron Rodgers

 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.

 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.

 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate clap;
extern crate dirs;
extern crate serde;

use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct GitId {
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct GitIds {
    ids: Vec<GitId>,
}

fn main() {
    let matches = App::new("Git ID")
        .version("0.1.0")
        .author("Cameron R. cameron.g.rodgers@gmail.com")
        .about("Manage emails and names for git repos.")
        .subcommand(SubCommand::with_name("list")
                .about("Lists the user.emails and user.names stored in the .gitid dotfile."))
        .subcommand(SubCommand::with_name("set") 
                .about("Set the user.email and user.name of the current git repo to the one specified. EG: gitid -s 3")
                .arg(Arg::with_name("number")
                     .required(true)
                     .index(1)))
        .subcommand(SubCommand::with_name("remove") // TODO: allow removing multiple ids at once?
                .about("Remove the specified user id.")
                .arg(Arg::with_name("number")
                     .required(true)
                     .index(1)))
        .subcommand(SubCommand::with_name("add")
                    .about("Add an email and name to the .gitid dotfile.")
                    .arg(Arg::with_name("email")
                         .long("email").short("e")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("name")
                         .long("name").short("n")
                         .required(true)
                         .takes_value(true)))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        match read_ids(String::from(".gitid")) {
            Some(ids) => {
                list_ids(ids);
            }
            None => {
                println!("No .gitid file found. Add an ID to create the file.");
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let email = matches.value_of("email").unwrap();
        let name = matches.value_of("name").unwrap();
        let new_id = GitId{ email: String::from(email), name: String::from(name)};
        let mut ids = read_ids(String::from(".gitid")).unwrap_or(Vec::new());
        if ! ids.contains(&new_id) {
            ids.push(new_id);
            ids.sort_by(|a, b| a.email.cmp(&b.email) );
        }
        let ids2 = ids.clone(); // TODO: Find better workaround rather than cloning
        write_ids(ids, String::from(".gitid"));
        list_ids(ids2);
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        let number = matches.value_of("number").unwrap();
        let number = match usize::from_str(number) {
            Ok(n) => n,
            Err(_) => {
                println!("Passed invalid argument.");
                return;
            },
        };
        match read_ids(String::from(".gitid")) {
            Some(ids) => {
                if number >= ids.len() {
                    println!("Invalid index.");
                    return;
                }
                let id = &ids[number];
                match Command::new("git")
                    .args(&["config", "user.email", &id.email])
                    .output()
                    {
                        Ok(_) => println!("Set email in current repo."),
                        Err(_) => println!("Failed to set email in current repo."),
                };
                match Command::new("git")
                    .args(&["config", "user.name", &format!("\"{}\"", id.name)])
                    .output()
                    {
                        Ok(_) => println!("Set name in current repo."),
                        Err(_) => println!("Failed to set name in current repo."),
                };
            }
            None => {
                println!("There were no ids stored in .gitid.");
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let number = matches.value_of("number").unwrap();
        let number = match usize::from_str(number) {
            Ok(n) => n,
            Err(_) => {
                println!("Passed invalid argument.");
                return;
            },
        };
        match read_ids(String::from(".gitid")) {
            Some(mut ids) => {
                if number >= ids.len() {
                    println!("Invalid index.");
                    return;
                }
                ids.remove(number);
                let ids2 = ids.clone();
                write_ids(ids, String::from(".gitid"));
                list_ids(ids2);
            }
            None => {
                println!("There were no ids stored in .gitid.");
            }
        }
    }
}

fn write_ids(ids: Vec<GitId>, filename: String) {
    let mut json = serde_json::to_string(&GitIds{ ids: ids}).unwrap();
    json.push('\n');
    let mut path = dirs::home_dir().unwrap();
    path.push(filename);
    let mut file = File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn read_ids(filename: String) -> Option<Vec<GitId>> {
    let mut path = dirs::home_dir().unwrap();
    path.push(filename);
    match fs::read_to_string(path) {
        Ok(contents) => {
            let ids: GitIds = serde_json::from_str(&contents).unwrap();
            Some(ids.ids)
        }
        Err(_) => None,
    }
}

fn list_ids(ids: Vec<GitId>) {
    println!("Git IDs:");
    if ids.len() == 0 {
        println!("no Git IDs have been added yet.");
    } else { 
        for (i, id) in ids.iter().enumerate() {
            println!("{}. Email: {}\n   Name : {}", i, id.email, id.name);
        }
    }
}
