mod cmake;
mod git;
mod new;
mod project;
mod utils;
mod zip;

use project::ProjectConfig;

use crate::new::create_new_project;
use cmake::update_cmake_set;
use git::clone_tag;
use seahorse::{App, Command, Context, Flag, FlagType};
use std::{env, fs};
use utils::{copy_dir_all, run_cross_platform};
use zip::unzip_file;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push("--help".to_string());
    }

    let new_cmd = Command::new("new")
        .description("Create a new Visual Scratch project")
        .usage("sebuilder new <project-name>")
        .action(new_project);

    let build_cmd = Command::new("build")
        .description("Build a project")
        .usage("sebuilder build [options]")
        .action(build_project)
        .flag(
            Flag::new("platform", FlagType::String)
                .description("Produced platform (3ds, wiiu, etc.)")
                .alias("p"),
        )
        .flag(
            Flag::new("debug", FlagType::Bool)
                .description("Enable debug logging")
                .alias("d"),
        );

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("sebuilder [args]")
        .command(new_cmd)
        .command(build_cmd);

    app.run(args);
}

fn build(c: &Context, name: &str, description: &str, author: &str, sb3_file: &str, platform: &str) {
    // Check if ./se exists, if so, delete it
    if std::path::Path::new("./se").exists() {
        std::fs::remove_dir_all("./se").expect("Failed to remove existing SE directory");
    }

    println!("Downloading SE...");
    clone_tag(
        "https://github.com/ScratchEverywhere/ScratchEverywhere.git",
        None,
        "./se",
    )
    .expect("Failed to clone SE repo");

    println!("Copying assets...");
    update_cmake_set("./se/CMakeLists.txt", "SE_APP_NAME", &name)
        .expect("Failed to update CMake set");
    update_cmake_set("./se/CMakeLists.txt", "SE_APP_DESCRIPTION", &description)
        .expect("Failed to update CMake set");
    update_cmake_set("./se/CMakeLists.txt", "SE_APP_AUTHOR", &author)
        .expect("Failed to update CMake set");

    std::fs::create_dir_all("./se/romfs/project").expect("Failed to create project directory");
    // unzip the sb3 file into the project directory
    unzip_file(&sb3_file, "./se/romfs/project").expect("Failed to unzip SB3 file");

    // Copy replace assets in ./se/gfx with assets/*
    std::fs::remove_dir_all("./se/gfx").expect("Failed to remove gfx directory");
    std::fs::create_dir_all("./se/gfx/").expect("Failed to create project directory");
    copy_dir_all("./assets", "./se/gfx").expect("Failed to copy assets.");

    println!("Building with Docker...");
    // Build the project
    run_cross_platform(
        format!(
            "docker build -f ./se/docker/Dockerfile.{} --target exporter -o . ./se",
            platform
        )
        .as_str(),
    ).expect("Failed to run cross-platform command");

    if c.bool_flag("debug") == false {
        fs::remove_dir_all("./se").expect("Failed to remove se directory");
    } else {
        // dont remove it :D
    }

    // Rename output
    fs::rename("./scratch-3ds.cia", format!("./{}.cia", name).as_str()).expect("Failed to rename scratch-3ds.cia");
    fs::rename("./scratch-3ds.3dsx", format!("./{}.3dsx", name).as_str()).expect("Failed to rename scratch-3ds.3dsx");

    println!("Done!");
}

fn new_project(c: &Context) {
    if c.args.is_empty() {
        eprintln!("You must provide a project name");
        return;
    }

    let name = &c.args[0];

    if let Err(e) = create_new_project(name) {
        eprintln!("Failed to create project: {}", e);
    }
}

fn build_project(c: &Context) {
    let config = ProjectConfig::load("sebuilder.toml").expect("Could not load project _config");

    let platform = c.string_flag("platform").unwrap_or_default();
    if platform.is_empty() {
        println!("Platform flag not set, defaulting to 3ds");
        build(
            c,
            &config.game.name,
            &config.game.description,
            &config.game.author,
            &config.assets.sb3,
            "3ds",
        );
        return;
    }

    build(
        c,
        &config.game.name,
        &config.game.description,
        &config.game.author,
        &config.assets.sb3,
        &platform,
    );
}
