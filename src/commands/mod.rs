mod utils;
pub mod ls;
pub mod pwd;
pub mod cat;

use std::io;
use outputhandler::OutputHandler;

pub struct BasicCommand<'a> {
    pub flags: Vec<&'a str>,
}

pub struct FileCommand<'a> {
    pub flags: Vec<&'a str>,
    pub files: Vec<&'a str>
}

pub struct ProgramCommand<'a> {
    pub cmd: &'a str, 
    pub flags: Vec<&'a str>, 
    pub args: Vec<&'a str>
}

///Holds all valid commands or none
pub enum Command<'a> {
    Ls(FileCommand<'a>),
    Pwd(BasicCommand<'a>),
    Cat(FileCommand<'a>),
    Exit,
    Program(ProgramCommand<'a>),
    Piped(Vec<Command<'a>>),
    Empty,
}

impl<'a> Command<'a> {
    pub fn new(input: &'a str) -> Command {
        let mut input = input.split(' ');
        if let Some(command) = input.next() {
            let is_flag = |i: &&str| i.starts_with("-");
            let input_args = input.clone();

            let flags = input.filter(is_flag).collect();
            let other_tokens = input_args.filter(|i| !is_flag(i)).collect();
            match command {
                "ls" => Command::Ls(FileCommand { flags, files: other_tokens, }),
                "pwd" => Command::Pwd(BasicCommand { flags }),
                "cat" => Command::Cat(FileCommand { flags, files: other_tokens, }),
                "exit" => Command::Exit,
                _ => Command::Program( ProgramCommand { cmd: command, flags, args: other_tokens }),
            }
        } else {
            Command::Empty
        }
    }
    pub fn execute<'b>(&self, oh: &'b mut OutputHandler) -> Result<&'b mut OutputHandler, io::Error> {
        match self {
            Command::Ls(file_cmd) => ls::execute(oh, file_cmd),
            Command::Cat(file_cmd) => cat::execute(oh, file_cmd),
            Command::Pwd(basic_cmd) => pwd::execute(oh, basic_cmd),
            _ => Ok(oh)
        }
    }

}