use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    let input_files = Arg::with_name("input files")
        .multiple(true)
        .required(true)
        .value_name("FILE")
        .help("Files to be converted");

    let strip = Arg::with_name("strip")
        .short("s")
        .long("strip")
        .help("Strip the file of metadata");

    App::new("EdC - EdJoPaTos Converter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("dry run")
                .long("dry-run")
                .global(true)
                .help("dont execute any commands and print them to stdout"),
        )
        .subcommand(
            SubCommand::with_name("versions")
                .about("Show versions of tools which are used by this tool (and if they are there in the first place)"),
        )
        .subcommand(
            SubCommand::with_name("photo")
                .visible_aliases(&["jpg", "image"])
                .about("jpg - Converts towards photos with many colors and without transparency")
                .arg(&strip)
                .arg(
                    Arg::with_name("resize")
                        .short("r")
                        .long("resize")
                        .help("Resize the image to fit inside an area. See --resize-size to change the default area"),
                )
                .arg(
                    Arg::with_name("resize size")
                        .long("resize-size")
                        .default_value("2000x1000>")
                        .help("Resize the image to fit inside a given area.\nhttps://imagemagick.org/script/command-line-options.php#resize"),
                )
                .arg(&input_files),
        )
        .subcommand(
            SubCommand::with_name("screenshot")
                .visible_aliases(&["png"])
                .about("png - Compresses pngs")
                .arg(&strip)
                .arg(
                    Arg::with_name("pedantic")
                        .long("pedantic")
                        .help("take considerably more effort to get small file size"),
                )
                .arg(&input_files),
        )
        .subcommand(
            SubCommand::with_name("sound")
                .visible_aliases(&["mp3"])
                .about("mp3 - extract or convert to mp3")
                .arg(&input_files),
        )
        .subcommand(
            SubCommand::with_name("video")
                .visible_aliases(&["mp4"])
                .about("mp4 - convert to mp4 video")
                .arg(&input_files),
        )
        .subcommand(
            SubCommand::with_name("gif-ish")
                .aliases(&["gifish", "gif"]) // alias gif might later change to create real gif files
                .about("mp4 - extract or convert to mp4 videos without sound")
                .arg(&input_files),
        )
}
