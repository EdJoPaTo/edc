#![forbid(unsafe_code)]

use std::path::Path;
use std::process::exit;

use command::Command;

mod cli;
mod command;
mod output_path;

#[allow(clippy::too_many_lines)]
fn main() {
    let matches = cli::build().get_matches();
    let dry_run = matches.is_present("dry run");

    if matches.subcommand_matches("versions").is_some() {
        println!("Check versions of all tools used...");
        check_version("convert", &["--version"]);
        check_version("ffmpeg", &["-version"]);
        check_version("mkdir", &["--version"]);
        check_version("oxipng", &["--version"]);

        exit(0);
    }

    let commands = match matches.subcommand() {
        ("photo", Some(matches)) => {
            let strip = matches.is_present("strip");
            let input_files = get_input_files(matches);

            let resize = if matches.is_present("resize") {
                matches.value_of("resize size")
            } else {
                None
            };

            let mut result = Vec::new();
            for file in input_files {
                let output = output_path::parse(file, "jpg").expect("failed to create output path");
                create_and_add_output_mkdir(&mut result, &output);

                let mut command = Command::new("convert");
                command.arg(file.to_str().unwrap());
                command.args(&["-background", "black", "-alpha", "remove"]);
                command.args(&["-sampling-factor", "4:2:0"]);

                if strip {
                    command.arg("-strip");
                }

                if let Some(resize) = resize {
                    command.arg("-resize");
                    command.arg(resize);
                }

                command.args(&["-quality", "85"]);
                command.arg(&output);

                result.push(command);
            }

            result
        }
        ("screenshot", Some(matches)) => {
            let pedantic = matches.is_present("pedantic");
            let strip = matches.is_present("strip");
            let input_files = get_input_files(matches);

            let mut result = Vec::new();
            for file in input_files {
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
        ("sound", Some(matches)) => {
            let mut result = Vec::new();
            for file in get_input_files(matches) {
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
        ("opus", Some(matches)) => {
            let mut result = Vec::new();
            for file in get_input_files(matches) {
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
        ("video", Some(matches)) => {
            let mut result = Vec::new();
            for file in get_input_files(matches) {
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
        ("gif-ish", Some(matches)) => {
            let mut result = Vec::new();
            for file in get_input_files(matches) {
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
        (name, Some(matches)) => {
            println!("Args: {:?}\n", matches);
            todo!("output target {}", name);
        }
        _ => panic!("requires subcommand"),
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

fn get_input_files<'a>(matches: &'a clap::ArgMatches) -> Vec<&'a Path> {
    let strings = matches
        .values_of("input files")
        .expect("couldnt read input files from command line")
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for file in strings {
        let path = Path::new(file);

        if path.is_absolute() {
            panic!("Absolute path is not supported: {}", file);
        }

        if !path.is_file() {
            panic!("A file needs to be a valid existing file: {}", file);
        }

        match path.to_str() {
            Some(path) => {
                if path.contains("../") {
                    panic!(
                        "paths need to be relative below the work directory: {}",
                        file
                    );
                }
            }
            None => panic!("only valid utf8 paths are supported: {}", file),
        }

        result.push(path);
    }

    result
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
