use std::os::args;

// const DEFAULT_PORT: int = 9000;
static DEFAULT_PORT: int = 9000;

fn usage() {
    println!("Usage: weers address <port>");
}

fn main1(hostname: &str) {
    run(hostname, DEFAULT_PORT);
}

fn main2(hostname: &str, port: &str) {
    let oPort: Option<int> = from_str(port);
    match oPort {
        Some(p) => run(hostname, p),
        None => usage(),
    }
}

fn run(hostname: &str, port: int) {
    println!("Ready to go on {}:{}", hostname, port);
}

fn main() {
    let args = args();

    match args.tail().len() {
        0 => unreachable!(),
        1 => main1(args[1]),
        2 => main2(args[1], args[2]),
        _ => usage(),
    }
}
