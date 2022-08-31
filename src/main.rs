use std::process;
use std::env;
use tmux_plugin::Config;
use tmux_plugin::Command;
use tmux_plugin::temp;
use tmux_plugin::cpu;

fn main() {
    env_logger::init();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        log::error!("Proplem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    log::info!("config: {:?}", config);
    match config.command {
        Command::Temp => { temp::run() },
        Command::Cpu => { cpu::average_cpu_usage() },
    }
}
