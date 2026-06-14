pub mod git;

use seahorse::{App, Context, Flag, FlagType};
use std::env;
use crate::git::clone_tag;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(default_action)
        .flag(
            Flag::new("name", FlagType::String)
                .description("name flag")
                .alias("n"),
        )
        .flag(
            Flag::new("description", FlagType::String)
                .description("description flag")
                .alias("d"),
        )
        .flag(
            Flag::new("author", FlagType::String)
                .description("author flag")
                .alias("a"),
        );

    app.run(args);
}

fn default_action(c: &Context) {

    // Example: calling your synchronous git module
    if let Ok(tag) = git::get_latest_se_tag() {
        println!("Latest SE tag: {}", tag);
    }

    // Process flags
    let name = c.string_flag("name").unwrap();
    let description = c.string_flag("description").unwrap();
    let author = c.string_flag("author").unwrap();

    clone_tag("https://github.com/ScratchEverywhere/ScratchEverywhere.git", None, "C:/Users/jones/Desktop/ScratchEverywhere").expect("Failed to clone SE repo");

}
