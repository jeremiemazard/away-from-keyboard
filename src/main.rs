use std::thread::sleep;
use clap::Parser;
use enigo::{Enigo, MouseControllable};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    seconds: u32,

    #[arg(short, long, default_value_t = 2)]
    pixels: i32,

    #[arg(short, long, default_value_t = false)]
    verbose: bool
}

fn main() {
    let args = Args::parse();

    let mut enigo = Enigo::new();
    let mut last_mouse_location = enigo.mouse_location();

    loop {
        sleep(Duration::from_secs(args.seconds as u64));

        let mut current_mouse_location = enigo.mouse_location();
        let afk = current_mouse_location == last_mouse_location;

        if afk {
            if args.verbose { println!("Idle. Moving mouse."); }
            enigo.mouse_move_relative(args.pixels, args.pixels);
            sleep(Duration::from_millis(20));
            current_mouse_location = enigo.mouse_location();
        } else if args.verbose {
            println!("User activity detected at {:?}", current_mouse_location);
        }

        last_mouse_location = current_mouse_location;
    }
}
