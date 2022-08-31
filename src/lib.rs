pub mod temp;
pub mod cpu;

#[derive(Debug)]
pub struct Config {
    pub command: Command,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a command"),
        };
        match command.as_str() {
            "temp" => Ok(Config { command: Command::Temp }),
            "cpu" => Ok(Config { command: Command::Cpu }),
            _ => return Err("Unrecognized command")
        }
    }
}
#[derive(Debug)]
pub enum Command {
    Temp,
    Cpu,
}
