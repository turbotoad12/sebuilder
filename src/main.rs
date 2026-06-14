pub mod git;

use seahorse::{App, Context};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(default_action);

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}
