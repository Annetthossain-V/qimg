
use std::env::args;
use std::io::{Error, Result, ErrorKind};
use std::path::Path;

#[allow(unused)]
pub struct Flags {
    pub files: Vec<String>,
    pub options: Vec<Options>,
    info: bool,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum Options {
    Help,
    Version,
    NewFile,
}

#[allow(unused)]
impl Flags {
    pub fn new() -> Self {
        Flags {
            files: Vec::new(),
            options: Vec::new(),
            info: false,
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        let args: Vec<String> = args().skip(1).collect();
        if (args.len() == 0) {
            self.options.push(Options::Help);
            self.info = true;
        }

        for arg in args {
            if (arg.chars().nth(0).unwrap() == '-') {
                match arg.as_str() {
                    "-h" | "--help" => { self.options.push(Options::Help); self.info = true; },
                    "-v" | "--version" => { self.options.push(Options::Version); self.info = true; },
                    "-nf" | "--new-file" => self.options.push(Options::NewFile),
                    _ => panic!("Unknown Option {}", arg),
                }
            } else {
                let file = Path::new(&arg);
                if (!file.exists()) {
                    return Err(Error::new(ErrorKind::NotFound, "File not found"));
                }
                self.files.push(arg);
            }
        }

        if (self.files.len() == 0 && !self.info) {
            return Err(Error::new(ErrorKind::InvalidInput, "No files specified"));
        }

        Ok(())
    }

    pub fn info(&self) {
        if (self.info) {
            for opt in &self.options {
                match opt {
                    Options::Help => println!("{HELP}"),
                    Options::Version => println!("{VERSION}"),
                    _ => continue,
                }
            }
            std::process::exit(0);
        }
    }

    pub fn contains(&self, item: Options) -> bool {
        for i in 0..self.options.len() {
            if self.options[i] == item {
                return true;
            }
        }
        return false;
    }
}


const VERSION: &str = "QImg version 0.1";
const HELP: &str = 
r#"usage <Options> <files>
Options:
 --help         # prints this message
 --version      # prints version info
 --new-file     # creates new image file instead of overriding existing one 
"#;
