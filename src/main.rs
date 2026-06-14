pub mod cmake;
pub mod git;
pub mod utils;

use crate::cmake::update_cmake_set;
use crate::git::clone_tag;
use crate::utils::run_cross_platform;
use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push("--help".to_string());
    }

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
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
        )
        .flag(
            Flag::new("sb3", FlagType::String)
                .description("SB3 file to build")
                .alias("s"),
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
    let sb3_file = c.string_flag("sb3").unwrap();

    clone_tag(
        "https://github.com/ScratchEverywhere/ScratchEverywhere.git",
        None,
        "./se",
    )
    .expect("Failed to clone SE repo");

    update_cmake_set("./se/CMakeLists.txt", "SE_APP_NAME", &name)
        .expect("Failed to update CMake set");
    update_cmake_set("./se/CMakeLists.txt", "SE_APP_DESCRIPTION", &description)
        .expect("Failed to update CMake set");
    update_cmake_set("./se/CMakeLists.txt", "SE_APP_AUTHOR", &author)
        .expect("Failed to update CMake set");

    std::fs::create_dir_all("./se/romfs/project").expect("Failed to create project directory");
    std::fs::copy(&sb3_file, "./se/romfs/project/project.sb3").expect("Failed to copy SB3 file");

    run_cross_platform(
        "docker build -f ./se/docker/Dockerfile.3ds --target exporter -o ./se ./se",
    ).expect("Failed to run cross-platform command");
}
