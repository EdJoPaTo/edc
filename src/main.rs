#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::exit;

use clap::Parser;
use command::Command;

use crate::cli::SubCommand;

mod cli;
mod command;
mod output_path;

#[allow(clippy::too_many_lines)]
fn main() {
    let matches = cli::Cli::parse();
    let dry_run = matches.dry_run;

    let commands = match matches.subcommand {
        SubCommand::Versions => {
            println!("Check versions of all tools used...");
            check_version("convert", &["--version"]);
            check_version("ffmpeg", &["-version"]);
            check_version("mkdir", &["--version"]);
            check_version("oxipng", &["--version"]);
            exit(0);
        }
        SubCommand::Photo {
            strip,
            input_files,
            resize,
            resize_size,
        } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "jpg").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("convert");
                command.arg(file.to_str().unwrap());
                command.args(&["-background", "black", "-alpha", "remove"]);
                command.args(&["-sampling-factor", "4:2:0"]);

                if strip {
                    command.arg("-strip");
                }

                if resize {
                    command.arg("-resize");
                    command.arg(&resize_size);
                }

                command.args(&["-quality", "85"]);
                command.arg(&output);

                result.push(command);
            }

            result
        }
        SubCommand::Screenshot {
            strip,
            input_files,
            pedantic,
        } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "png").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("oxipng");

                if pedantic {
                    command.arg("-Z");
                }

                if strip {
                    command.args(&["--strip", "safe"]);
                }

                command.arg(file.to_str().unwrap());
                command.arg("--out");
                command.arg(&output);

                result.push(command);
            }

            result
        }
        SubCommand::Sound { input_files } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "mp3").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("ffmpeg");
                command.args(&["-v", "error"]);
                command.arg("-stats");
                command.arg("-vn");
                command.arg("-i");
                command.arg(file.to_str().unwrap());
                command.arg(&output);

                result.push(command);
            }

            result
        }
        SubCommand::Opus { input_files } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "ogg").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("ffmpeg");
                command.args(&["-v", "error"]);
                command.arg("-stats");
                command.arg("-vn");
                command.arg("-i");
                command.arg(file.to_str().unwrap());
                command.args(&["-c:a", "libopus"]);
                command.arg(&output);

                result.push(command);
            }

            result
        }
        SubCommand::Video { input_files } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "mp4").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("ffmpeg");
                command.args(&["-v", "error"]);
                command.arg("-stats");
                command.arg("-i");
                command.arg(file.to_str().unwrap());
                command.arg(&output);

                result.push(command);
            }

            result
        }
        SubCommand::Gifish { input_files } => {
            check_input_files(&input_files);
            let mut result = Vec::new();
            for file in &input_files {
                let output = output_path::parse(file, "mp4").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("ffmpeg");
                command.args(&["-v", "error"]);
                command.arg("-stats");
                command.arg("-an");
                command.arg("-i");
                command.arg(file.to_str().unwrap());
                command.arg(&output);

                result.push(command);
            }

            result
        }
    };

    if dry_run {
        for command in commands {
            println!("{}", command.to_command_line());
        }
    } else {
        for (i, command) in commands.iter().enumerate() {
            println!(
                "Run ({:>4}/{}): {}",
                i + 1,
                commands.len(),
                command.to_command_line()
            );

            let status = command
                .to_std_command()
                .status()
                .expect("failed to execute process");

            assert!(status.success());
        }
    }
}

fn check_input_files(paths: &[PathBuf]) {
    for path in paths {
        let path = path;
        assert!(
            !path.is_absolute(),
            "Absolute path is not supported: {:?}",
            path
        );

        assert!(
            path.is_file(),
            "Input file needs to be a valid existing file: {:?}",
            path
        );

        match path.to_str() {
            Some(path) => {
                assert!(
                    !path.contains("../"),
                    "Paths need to be relative below the work directory: {:?}",
                    path
                );
            }
            None => panic!("Only valid utf8 paths are supported: {:?}", path),
        }
    }
}

fn create_and_add_output_mkdir(commands: &mut Vec<Command>, output_file: &str) {
    let mkdir = output_path::create_folder_for_file_path_command(Path::new(&output_file))
        .expect("failed to create output path folder command");
    if !commands.contains(&mkdir) {
        commands.push(mkdir);
    }
}

fn check_version(program: &str, args: &[&str]) {
    println!("\n\ncheck {}...", program);
    match std::process::Command::new(program).args(args).output() {
        Ok(output) => {
            if !output.status.success() {
                println!("Statuscode: {}", output.status);
            }

            if !output.stdout.is_empty() {
                match String::from_utf8(output.stdout) {
                    Ok(stdout) => println!("{}", stdout),
                    Err(err) => println!("Error parsing stdout: {}", err),
                }
            }

            if !output.stderr.is_empty() {
                match String::from_utf8(output.stderr) {
                    Ok(stderr) => println!("{}", stderr),
                    Err(err) => println!("Error parsing stderr: {}", err),
                }
            }
        }

        Err(err) => println!("Failed to check version: {}", err),
    }
}
