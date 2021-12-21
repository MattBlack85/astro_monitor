// use std::process::Command;
// use std::process::Stdio;
// use std::io::Write;


// fn main() {

//     let ps_proc = Command::new("ps")
// 	.arg("aux")
// 	.output()
// 	.expect("ps command failed");
 
//     let grep_proc = Command::new("grep")
// 	.arg("kstars")
// 	.stdin(Stdio::piped())
// 	.stdout(Stdio::piped())
// 	.spawn()
// 	.expect("grep failed");
    
//     grep_proc.stdin.unwrap().write_all(&ps_proc.stdout);

//     let mut s = String::new();
//     match grep_proc.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}", why),
//         Ok(_) => print!("wc responded with:\n{}", s),
//     }

//     println!("{}", s);
// }

use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    // List all running processes via ps aux
    let ps_proc = match Command::new("ps")
 	.arg("aux")
 	.output() {
	    Ok(process) => process,
	    Err(reason) => panic!("ps aux failed because: {}", reason),
	};

    // Spawn the `grep` command
    let process = match Command::new("grep")
	                         .arg("kstars")
                                 .stdin(Stdio::piped())
                                 .stdout(Stdio::piped())
                                 .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // Write `ps aux` output to the `stdin` of `grep`
    match process.stdin.unwrap().write_all(&ps_proc.stdout) {
        Err(reason) => panic!("could not write to grep stdin: {}", reason),
        Ok(_) => (),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
    println!("{}", s);
}
