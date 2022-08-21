use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[clap(about, author, version, name = "EdC - EdJoPaTos Converter")]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,

    /// dont execute any commands and print them to stdout
    #[clap(long, global = true)]
    pub dry_run: bool,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Show versions of tools which are used by this tool (and if they are there in the first place)
    Versions,

    /// jpg - Converts towards photos with many colors and without transparency
    #[clap(visible_alias = "jpg", visible_alias = "photo")]
    Photo {
        /// Strip the file of metadata
        #[clap(short, long)]
        strip: bool,

        /// Resize the image to fit inside an area.
        ///
        /// See --resize-size to change the default area.
        #[clap(short, long)]
        resize: bool,

        /// Resize the image to fit inside a given area.
        ///
        /// <https://imagemagick.org/script/command-line-options.php#resize>
        #[clap(long, value_hint = ValueHint::Other, default_value = "2000x1000>")]
        resize_size: String,

        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },

    /// png - Compresses pngs
    #[clap(visible_alias = "png")]
    Screenshot {
        /// Strip the file of metadata
        #[clap(short, long)]
        strip: bool,

        /// take considerably more effort to get small file size
        #[clap(short, long)]
        pedantic: bool,

        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },

    /// mp3 - extract or convert to mp3
    #[clap(visible_alias = "mp3")]
    Sound {
        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },

    /// ogg - extract or convert to opus encoded audio file
    #[clap(visible_alias = "ogg")]
    Opus {
        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },

    /// mp4 - convert to mp4 video
    #[clap(visible_alias = "mp4")]
    Video {
        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },

    /// mp4 - extract or convert to mp4 videos without sound
    #[clap(visible_alias = "gif")]
    Gifish {
        /// Files to be converted
        #[clap(value_hint = ValueHint::FilePath, value_name = "FILE")]
        input_files: Vec<PathBuf>,
    },
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
