//! ShellSpeed
//!
//! Shellspeed is an application that attempts to indicate the performance of a shell by measuring the
//! speed that a set of specified or default commands take to execute.
//!
//! Currently, the best way to run this application is to allow it to run its default set of commands.
//! You specify the shell you want to test, and the net time to run these commands will be returned:
//!
//! ./shellspeed -s ion -d
//!
//! You can also specify a specific command to run:
//! ./shellspeed -s ion -c "ls"
//!
//!However, because one command can take such a short time, often you will se a net time of negative
//! due to the applications subtracting from a baseline of invoking the shell with no commands.
//!
//! Shellspeed application help
//!
//!The command line help can be seen by typing:
//!
//!./shellspeed -h
//!
//! Written by Daniel Gibson, Jan 6, 2020
//! Copywrite 2020 Daniel J Gibson
//!
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
use std::process::{exit, Command, Output};

mod switches;
use crate::switches::Switches;

fn main() {
    println!("Welcome to Shellspeed");
    let switches = Switches::new();
    //println!("switches.shell: {}, switches.command: {}", switches.shell, switches.command);
    //let now = time::precise_time_ns();
    if switches.shell.len() == 0 {
        println!("Error: A shell must be specified with the -s switch!");
        exit(0);
    };

    let nu_script_snippet = "; k=$((i+1)); amt=$((k*i))";
    let ion_script_snippet = "; let k=$((i+1)); let amt=$((k*i))";
    let mut default_script = String::from("");
    let size = 100;

    if switches.default {
        match switches.shell.as_str() {
            "nu" => {
                default_script += "i=1";
                for _i in 0..size {
                    default_script += nu_script_snippet;
                }
            }
            "ion" => {
                default_script += "let i=1";
                for _i in 0..size {
                    default_script += ion_script_snippet;
                }
            }
            _ => {
                // Used for all other shells, including bash
                default_script += "i=1";
                for _i in 0..size {
                    default_script += nu_script_snippet;
                }
            }
        };
    };

    let mut total_shell_empty_duration: time::Duration = time::Duration::new(0, 0);
    let mut total_shell_cmd_duration: time::Duration = time::Duration::new(0, 0);

    let limit = 10;

    for _count in 0..limit {
        let (empty_duration, shell_output) = test_empty_shell_cmd(&switches);
        //if let Some(output) = shell_output {
        //    println!("Empty shell command output status: {:?}", output.status);
        //};

        let (duration, shell_output) = test_shell_cmd(&switches, &default_script);
        //if let Some(output) = shell_output {
        //    println!("Shell command output status: {:?}", output.status);
        //};

        total_shell_empty_duration += empty_duration;
        total_shell_cmd_duration += duration;
    } // end for

    let average_empty_duration: time::Duration = total_shell_empty_duration / 10;
    let average_shell_cmd_duration: time::Duration = total_shell_cmd_duration / 10;

    println!(
        "\nEmpty duration in {} ns",
        average_empty_duration.whole_nanoseconds()
    );
    println!(
        "Shell plus commands duration in {} ns",
        average_shell_cmd_duration.whole_nanoseconds()
    );
    println!(
        "\nNet shell commands duration in {} ns",
        (average_shell_cmd_duration - average_empty_duration).whole_nanoseconds()
    );
}
/// Tests the speed of running a set of shell commands.
fn test_shell_cmd(
    switches: &Switches,
    default_script: &String,
) -> (time::Duration, Option<Output>) {
    let now = time::OffsetDateTime::now_utc();
    let final_output: Option<Output>;
    if switches.cmd_switch {
        //println!("Shell Command provided and the command is: {}", switches.command);
        let output = Command::new(&switches.shell)
            .arg("-c")
            .arg(&switches.command)
            //.arg(shell_cmd) // *** Maybe because of space between -c and arg, I have to use two .args ?
            .output()
            .expect(&format!(
                "Error: Failed to execute {} shell!",
                &switches.shell
            ));
        final_output = Some(output);
    //let duration = time::OffsetDateTime::now_utc() - now;
    //println!("Output status: {:?}, Length of returned results: {}", output.status, output.stdout.len());
    //println!("Completing in {} ns", duration.whole_nanoseconds());
    } else {
        if switches.default {
            let output = Command::new(&switches.shell)
                .arg("-c")
                //.arg("for i in {1..100000}; do { k=$((i+1)); amt=$((k*i)); } done;")
                .arg(default_script)
                //.arg(shell_cmd) // *** Maybe because of space between -c and arg, I have to use two .args ?
                .output()
                .expect(&format!(
                    "Error: Failed to execute {} shell!",
                    &switches.shell
                ));
            final_output = Some(output);
        } else {
            //let shell_cmd = ("-c ".to_owned().push_str(&switches.command)).to_string();
            //let shell_cmd = format!("-c \"{}\"", &switches.command);
            //println!("shell_cmd: {}", shell_cmd);
            //let output = Command::new("nu")

            let output = Command::new(&switches.shell)
                .arg("-c")
                .arg("")
                //.arg(shell_cmd) // *** Maybe because of space between -c and arg, I have to use two .args ?
                .output()
                .expect(&format!(
                    "Error: Failed to execute {} shell!",
                    &switches.shell
                ));
            //let duration = time::OffsetDateTime::now_utc() - now;
            //let test:Output = output;
            final_output = Some(output);
        };
    }
    (time::OffsetDateTime::now_utc() - now, final_output)
}
/// Tests the speed of running a shell without any shell commands.
fn test_empty_shell_cmd(switches: &Switches) -> (time::Duration, Option<Output>) {
    let now = time::OffsetDateTime::now_utc();
    let final_output: Option<Output>;
    let output = Command::new(&switches.shell)
        .arg("-c")
        .arg("")
        //.arg(shell_cmd) // *** Maybe because of space between -c and arg, I have to use two .args ?
        .output()
        .expect(&format!(
            "Error: Failed to execute {} shell!",
            &switches.shell
        ));
    //let duration = time::OffsetDateTime::now_utc() - now;
    final_output = Some(output);
    //final_output = output;
    (time::OffsetDateTime::now_utc() - now, final_output)
}
