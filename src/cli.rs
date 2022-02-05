use clap::{app_from_crate, App, AppSettings, ValueHint, Arg};

#[must_use]
pub fn build() -> App<'static> {
    let input_files = Arg::new("input files")
        .value_name("FILE")
        .value_hint(ValueHint::FilePath)
        .validator(validate_input_file)
        .multiple_values(true)
        .required(true)
        .help("Files to be converted");

    let strip = Arg::new("strip")
        .short('s')
        .long("strip")
        .help("Strip the file of metadata");

    app_from_crate!()
        .name("EdC - EdJoPaTos Converter")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::new("dry run")
                .long("dry-run")
                .global(true)
                .help("dont execute any commands and print them to stdout"),
        )
        .subcommand(
            App::new("versions")
                .about("Show versions of tools which are used by this tool (and if they are there in the first place)"),
        )
        .subcommand(
            App::new("photo")
                .visible_aliases(&["jpg", "image"])
                .about("jpg - Converts towards photos with many colors and without transparency")
                .arg(&strip)
                .arg(
                    Arg::new("resize")
                        .short('r')
                        .long("resize")
                        .help("Resize the image to fit inside an area. See --resize-size to change the default area"),
                )
                .arg(
                    Arg::new("resize size")
                        .long("resize-size")
                        .default_value("2000x1000>")
                        .value_hint(ValueHint::Other)
                        .takes_value(true)
                        .help("Resize the image to fit inside a given area.\nhttps://imagemagick.org/script/command-line-options.php#resize"),
                )
                .arg(&input_files),
        )
        .subcommand(
            App::new("screenshot")
                .visible_aliases(&["png"])
                .about("png - Compresses pngs")
                .arg(&strip)
                .arg(
                    Arg::new("pedantic")
                        .long("pedantic")
                        .help("take considerably more effort to get small file size"),
                )
                .arg(&input_files),
        )
        .subcommand(
            App::new("sound")
                .visible_aliases(&["mp3"])
                .about("mp3 - extract or convert to mp3")
                .arg(&input_files),
        )
        .subcommand(
            App::new("opus")
                .aliases(&["ogg"])
                .about("ogg - extract or convert to opus encoded audio file")
                .arg(&input_files),
        )
        .subcommand(
            App::new("video")
                .visible_aliases(&["mp4"])
                .about("mp4 - convert to mp4 video")
                .arg(&input_files),
        )
        .subcommand(
            App::new("gif-ish")
                .aliases(&["gifish", "gif"]) // alias gif might later change to create real gif files
                .about("mp4 - extract or convert to mp4 videos without sound")
                .arg(&input_files),
        )
}

fn validate_input_file(file: &str) -> Result<(), String> {
    let path = std::path::Path::new(file);

    if path.is_absolute() {
        return Err(format!("Absolute path is not supported: {}", file));
    }

    if !path.is_file() {
        return Err(format!(
            "Input file needs to be a valid existing file: {}",
            file
        ));
    }

    match path.to_str() {
        Some(path) => {
            if path.contains("../") {
                return Err(format!(
                    "Paths need to be relative below the work directory: {}",
                    file
                ));
            }
        }
        None => return Err(format!("Only valid utf8 paths are supported: {}", file)),
    }

    Ok(())
}

#[test]
fn verify_app() {
    build().debug_assert();
}
