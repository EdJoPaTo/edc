use std::path::Path;

use crate::command::Command;

pub fn parse(input: &Path, extension: &str) -> Option<String> {
    let output_folder = input
        .parent()
        .and_then(Path::to_str)
        .filter(|s| !s.is_empty())
        .map_or_else(
            || String::from("converted"),
            |parent| format!("converted/{}", parent),
        );
    let basename = input.file_stem()?.to_str()?;
    Some(format!("{}/{}.{}", output_folder, basename, extension))
}

pub fn create_folder_for_file_path_command(path: &Path) -> Option<Command> {
    let folder = path.parent()?.to_str()?;

    if folder.is_empty() {
        None
    } else {
        let mut command = Command::new("mkdir");
        command.arg("-p");
        command.arg(folder);
        Some(command)
    }
}

#[test]
fn parse_simple() {
    assert_eq!(
        parse(Path::new("foo.bar"), "txt"),
        Some("converted/foo.txt".to_string()),
    );
}

#[test]
fn parse_relative_folder() {
    assert_eq!(
        parse(Path::new("bla/foo.bar"), "txt"),
        Some("converted/bla/foo.txt".to_string()),
    );
}

#[test]
fn mkdir() {
    assert_eq!(
        create_folder_for_file_path_command(Path::new("something.txt")),
        None
    );
    assert_eq!(
        create_folder_for_file_path_command(Path::new("foo/something.txt"))
            .map(|o| o.to_command_line()),
        Some("mkdir -p foo".to_string())
    );
}
