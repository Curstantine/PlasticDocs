mod config;
mod constants;
mod writer;

fn main() {
    let conf = config::read_config(None);

    println!("Hello, world!");
}
