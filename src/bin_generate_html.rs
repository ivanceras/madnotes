mod serve_files;
mod server;
use sauron::prelude::*;

fn main() {
    let settings = client::Settings::default();
    let html = server::page::index(&settings).render_to_string();
    println!("{}", html);
}
