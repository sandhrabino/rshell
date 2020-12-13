use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::path::Path;
use std::env;
use std::io::{self, Read, Write};
use std::process;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut file = File::create("history")?;
    let mut n=0;
    file.write_all(b"Hello, world!")?;
    println!("Hello, Welcome to my shell!");
    println!("=========== Please execute only one command at a time ==========");
    while n<=10 {
        let mut command_buffer = String::new();
        let path = env::current_dir()?;
        println!("@user : >");
//        print!("@{} :{} >",user.name().to_string_lossy(), path.display());
        io::stdin().read_line(&mut command_buffer).ok().expect("Failed");
        file.write_all(command_buffer.as_bytes()).expect("Failed");
        let mut entire_command = command_buffer.trim().split_whitespace();
        //Command::new(&iter[0]).args(&iter[1]).spawn().expect("Failed ");

        let command = entire_command.next().unwrap();
        let args = entire_command;


        let output = Command::new(command)
                     .args(args)
                     .output()
                     .expect("failed");
        if command_buffer == "exit" {
            process::exit(0);
        }
        println!("stdout: >> {}", String::from_utf8_lossy(&output.stdout));
        // Increment counter
        n += 1;
    }
    Ok(())
}
