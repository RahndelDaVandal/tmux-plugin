use std::{
    fs::File,
    io::{prelude::*, BufReader},
    thread,
    time,
};

pub fn run() {
    let delay = time::Duration::from_millis(2000);
    loop{
        let t = get_cpu_temp();
        println!("{t:.1}");
        thread::sleep(delay);
    }
}

pub fn get_cpu_temp() -> f32 {
    let file_path = "/sys/class/thermal/thermal_zone0/temp";
    let mut temp: f32 = 0.0;
    match File::open(file_path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();
            contents.pop();
            temp = contents.parse().unwrap();
        }
        Err(e) => eprint!("Error getting cpu temp: {e}"),
    }
    temp / 1000.0
}
