pub mod cmake;
pub mod git;
pub mod utils;
pub mod zip;

use crate::cmake::update_cmake_set;
use crate::git::clone_tag;
use crate::utils::run_cross_platform;
use seahorse::{App, Context, Flag, FlagType};
use std::env;
use crate::zip::unzip_file;

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
        )
        .flag(
            Flag::new("platform", FlagType::String)
                .description("Produced platform (3ds, wiiu, etc.)")
                .alias("p"),
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
    // Check if the platform flag is set
    let mut platform = c.string_flag("platform").unwrap_or_default();
    if platform.is_empty() {
        println!("Platform flag not set, defaulting to 3ds");
        platform.push_str("3ds");
    }

    // Check if ./se exists, if so delete it
    if std::path::Path::new("./se").exists() {
        std::fs::remove_dir_all("./se").expect("Failed to remove existing SE directory");
    }
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
    // unzip the sb3 file into the project directory
    unzip_file(&sb3_file, "./se/romfs/project").expect("Failed to unzip SB3 file");

    // Build the project
    run_cross_platform(
        format!("docker build -f ./se/docker/Dockerfile.{} --target exporter -o ./se ./se", platform).as_str(),
    ).expect("Failed to run cross-platform command");
}
