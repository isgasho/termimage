//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use termimage::Options;
//! let options = Options::parse();
//! println!("Image to display: {}", options.image.0);
//! ```


use clap::{App, Arg, AppSettings};
use std::path::PathBuf;
use std::str::FromStr;
use regex::Regex;
use term_size;
use std::fs;


lazy_static! {
    static ref SIZE_ARG_RGX: Regex = Regex::new(r"(\d+)[xX](\d+)").unwrap();
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Image file to display.
    pub image: (String, PathBuf),
    /// Output size. Default: detected from terminal size or no default.
    pub size: (u32, u32),
    /// Whether to preserve the image's aspect ratio when resizing. Default: `true`.
    pub preserve_aspect: bool,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let szarg_def;
        let mut szarg = Arg::from_usage("-s --size [size] 'Image file to display'").validator(Options::size_validator);
        if let Some((w, h)) = term_size::dimensions() {
            szarg_def = format!("{}x{}", w, h);
            szarg = szarg.default_value(&szarg_def);
        }

        let matches = App::new("termimage")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .about("Display images in your terminal, kind of")
            .arg(Arg::from_usage("<IMAGE> 'Image file to display'").validator(Options::image_file_validator))
            .arg(szarg)
            .arg(Arg::from_usage("-f --force 'Don\\'t preserve the image\\'s aspect ratio'"))
            .get_matches();

        let image = matches.value_of("IMAGE").unwrap();
        Options {
            image: (image.to_string(), fs::canonicalize(image).unwrap()),
            size: Options::parse_size(matches.value_of("size").unwrap()).unwrap(),
            preserve_aspect: !matches.is_present("force"),
        }
    }

    fn parse_size(s: &str) -> Option<(u32, u32)> {
        SIZE_ARG_RGX.captures(s).map(|c| (u32::from_str(c.at(1).unwrap()).unwrap(), u32::from_str(c.at(2).unwrap()).unwrap()))
    }

    fn image_file_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Image file \"{}\" not found", s))
    }

    fn size_validator(s: String) -> Result<(), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err(format!("Can't resize image to size 0")),
            Some(_) => Ok(()),
        }
    }
}
