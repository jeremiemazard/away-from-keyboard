use std::thread::sleep;
use clap::Parser;
use enigo::{Enigo, MouseControllable};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    seconds: u32,

    #[arg(short, long, default_value_t = 1)]
    pixels: i32,

    #[arg(short, long, default_value_t = false)]
    verbose: bool
}

fn main() {
    let args = Args::parse();

    let mut enigo = Enigo::new();
    let mut last_mouse_location = enigo.mouse_location();
    let mut sign: i32 = 1;

    loop {
        sleep(Duration::from_secs(args.seconds as u64));

        let mut current_mouse_location = enigo.mouse_location();
        let afk = current_mouse_location == last_mouse_location;

        if afk {
            if args.verbose { println!("Idle. Moving mouse."); }
            enigo.mouse_move_relative(sign * args.pixels, 0);
            sleep(Duration::from_millis(20));
            current_mouse_location = enigo.mouse_location();
        } else if args.verbose {
            println!("Activity detected at {:?}", current_mouse_location);
        }

        sign *= -1;
        last_mouse_location = current_mouse_location;
    }
}
