pub mod prompts;

use std::{ffi::OsStr, env, io};

pub fn cmd<I, S>(cmd: &str, args: I) -> io::Result<String>
where
    I: IntoIterator<Item = S> + std::fmt::Debug,
    S: AsRef<OsStr>,
{
    let raw = std::process::Command::new(cmd)
        .args(args)
        .output()?
        .stdout;
    Ok(std::str::from_utf8(&raw).unwrap().trim().to_string())
}

pub fn format_prompt(prompt: String) -> String {
    let mut out = prompt
        .replace("%{", "")
        .replace("%}", "")
        .replace("\x01", "")
        .replace("\x02", "");

    // Various censors for privacy and to make the finished prompt as generic as possible.
    if let Ok(host) = cmd("hostname", [] as [&str; 0]) {
        if let Some(host) = host.split(".").next() {
            out = out.replace(host, "host");
        };
    };

    if let Ok(user) = env::var("USER") {
        out = out.replace(&user, "user");
    };

    if let Ok(branch) = cmd("git", ["branch", "--show-current"]) {
        out = out.replace(&branch, "branch");
    };

    if let Ok(pwd) = env::var("PWD") {
        out = out.replace(&pwd, "/foo/bar");

        let mut iter = pwd.split("/");

        iter.next();

        for segment in iter {
            out = out.replace(segment, "dir");
        }
    }

    let trimmed = out.trim();

    trimmed.to_string()
}
