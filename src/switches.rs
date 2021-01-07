//!
//! Switches module that handles interpretation of command line switches so the main program
//! can respond correctly.  Also provides help switch functionality.
//!
//! Written by: Daniel Gibson. Jan 6, 2020.
//! Copywrite 2020 Daniel J Gibson
//!
// This file is part of Shellspeed.
//
// Shellspeed is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Shellspeed is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::process;

#[derive(Clone)]
pub struct Switches {
    pub shell: String,
    pub cmd_switch: bool,
    pub command: String,
    pub default: bool,
}
impl Switches {
    pub fn new() -> Switches {
        let args: Vec<String> = env::args().collect();
        if let Some(_helps) = args.iter().find(|x| x.starts_with("-h")) {
            print_help_quit();
        };
        if let Some(_helps) = args.iter().find(|x| x.starts_with("--help")) {
            print_help_quit();
        };

        let mut shell = String::from("");
        if let Some(shell_pos) = args.iter().position(|x| x.starts_with("-s")) {
            if args[shell_pos].len() > 2 {
                shell = args[shell_pos].clone()[2..].to_string();
            } else {
                shell = args[shell_pos + 1].clone();
            }
        };

        let mut command = String::from("");
        let mut cmd_switch = false;
        if let Some(_cmd_switch_ind) = args.iter().find(|x| x.starts_with("-c")) {
            cmd_switch = true;

            if let Some(shell_pos) = args.iter().position(|x| x.starts_with("-c")) {
                if args[shell_pos].len() > 2 {
                    command = args[shell_pos].clone()[2..].to_string();
                } else {
                    command = args[shell_pos + 1].clone();
                }
            }
        }; // if let Some(cmd_switch_ind) = args.iter().find(|x| x.starts_with("-c")) {

        let mut default = false;

        if let Some(_cmd_switch_ind) = args.iter().find(|x| x.starts_with("-d")) {
            default = true;
        };

        Switches {
            shell,
            cmd_switch,
            command,
            default,
        }
    }
}
/// Provides functionality for the help switch (-h or --help).
fn print_help_quit() {
    println!("\nShellspeed Help:");
    println!("This app measures the speed of a set of specified or default commands in a specified shell.
\n It does this by sending a stream of shell script commands using the -c switch in the shell command.
\n Since not all shell commands allow loops, at this time, it creates a stream of repetative shell commands
\n starting out with let i=0 for ion, and i=0 for nu and all others, then:
\n
\n ion:               \"; let k=$((i+1)); let amt=$((k*i))\"
\n nu and all others: \"; k=$((i+1)); amt=$((k*i))\"
\n
\n It averages 10 calls of the shell with no script commands, and 10 calls of the shell with the script above, then subtracts the
\n averages to get a net measurement of how long it takes to run the script.
\n
eg: \n");
    println!("
shellspeed -h  for help
shellspeed --help  for help
shellspeed -s ion -d  to measure the time to execute a default shell script: 
shellspeed -s ion -c \"ls\" or \"for i in {{1..10000}}; do {{ let k=$((i+1)); let amt=$((k*i)); }} done;\" for example to measure the time to execute the ls command in the ion shell.
                 ");
    process::exit(0);
}
