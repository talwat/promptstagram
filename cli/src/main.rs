use std::fs;

use cli::{cmd, format_prompt, prompts::Prompt};

fn main() {
    // TODO: Add code for authentication and uploading.

    let shell = cmd("ps", ["-p", &std::os::unix::process::parent_id().to_string(), "-o", "comm="]).unwrap();
    
    let shells = fs::read_to_string("/etc/shells").unwrap();
    if !shells.contains(&shell) {
        panic!("you must run this program from a valid shell")
    }

    let output = cmd(&shell, ["-i", "-c", "echo $PS1"]).unwrap();
    let subst = cmd(&shell, ["-i", "-c", &format!("echo \"{}\"", output)]).unwrap();

    let prompt = format_prompt(subst);

    println!("{}", prompt);
    let prompt = Prompt::new(&prompt);

    //dbg!(prompt);
}
