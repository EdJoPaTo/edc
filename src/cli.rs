use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    let input_files = Arg::with_name("input files")
        .multiple(true)
        .required(true)
        .value_name("FILE")
        .help("Files to be converted");

    let strip = Arg::with_name("strip")
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
            SubCommand::with_name("photo")
                .visible_aliases(&["jpg", "image"])
                .about("jpg - Converts towards photos with many colors and without transparency")
                .arg(&strip)
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
}
