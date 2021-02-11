use std::path::Path;

use command::Command;

mod cli;
mod command;
mod output_path;

fn main() {
    let matches = cli::build().get_matches();
    let dry_run = matches.is_present("dry run");

    let commands = match matches.subcommand() {
        ("photo", Some(matches)) => {
            let strip = matches.is_present("strip");
            let input_files = get_input_files(&matches);

            let resize = if matches.is_present("resize") {
                matches.value_of("resize size")
            } else {
                None
            };

            let mut result = Vec::new();
            for file in input_files {
                let output = output_path::parse(file, "jpg").expect("failed to create output path");
                let mkdir = output_path::create_folder_for_file_path_command(Path::new(&output))
                    .expect("failed to create output path folder command");
                if !result.contains(&mkdir) {
                    result.push(mkdir);
                }

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
            let input_files = get_input_files(&matches);

            let mut result = Vec::new();
            for file in input_files {
                let output = output_path::parse(file, "png").expect("failed to create output path");
                let mkdir = output_path::create_folder_for_file_path_command(Path::new(&output))
                    .expect("failed to create output path folder command");
                if !result.contains(&mkdir) {
                    result.push(mkdir);
                }

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

        result.push(path);
    }

    result
}
