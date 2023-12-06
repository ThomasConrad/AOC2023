use std::fmt::Display;
/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::io::Write;
use std::process;
use std::{io, process::Command};

pub fn submit(
    day: u32,
    part: u32,
    solver: impl FnOnce(&str) -> Option<Box<dyn Display>>,
    input: &str,
) {
    let answer = solver(input);
    match &answer {
        Some(answer) => {
            println!("Submitting answer: {}", answer);
        }
        None => {
            panic!("Couldn't solve")
        }
    }
    // check if aoc binary exists and is callable.
    if Command::new("aoc").arg("-V").output().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    let mut cmd_args = vec![];
    if let Ok(year) = std::env::var("YEAR") {
        cmd_args.push("--year".into());
        cmd_args.push(year.to_string());
    }
    cmd_args.append(&mut vec![
        "--day".into(),
        day.to_string(),
        "submit".into(),
        part.to_string(),
    ]);

    println!(
        "Submitting output with >aoc {} {}",
        cmd_args.join(" "),
        answer.unwrap()
    );

    match Command::new("aoc").args(cmd_args).output() {
        Ok(cmd_output) => {
            io::stderr()
                .write_all(&cmd_output.stderr)
                .expect("could not write cmd stdout to pipe.");
            io::stdout()
                .write_all(&cmd_output.stdout)
                .expect("could not write cmd stderr to pipe.");
            if !cmd_output.status.success() {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            process::exit(1);
        }
    }
}
