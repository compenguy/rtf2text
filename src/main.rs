use std::{fs, io, path};

use anyhow::{Context, Result};
use flexi_logger::{detailed_format, Logger};
use log::debug;

mod rtf_control;
mod rtftotext;

fn main() -> Result<()> {
    let app = clap::command!("")
        .setting(clap::AppSettings::ColorAuto)
        .setting(clap::AppSettings::ColoredHelp)
        .arg(clap::Arg::with_name("input-file")
            .help("Filename of Rich Text File to convert to text, or leave unset to read from stdin")
            .short('i')
            .long("input-file")
            .takes_value(true)
            .required(true)
            .value_name("INPUT-FILE"))
        .arg(clap::Arg::with_name("output-file")
            .help("Filename write the extracted text to, or leave unset to print to stdout")
            .short('o')
            .long("output-file")
            .takes_value(true)
            .required(true)
            .value_name("OUTPUT-FILE"))
        .arg(clap::Arg::with_name("debug")
            .short('g')
            .long("debug")
            .multiple(true)
            .hidden(true)
            .help("Enable debug-level output"));

    let matches = app.get_matches();

    let crate_log_level = match matches.occurrences_of("debug") {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    let general_log_level = match crate_log_level {
        log::LevelFilter::Trace | log::LevelFilter::Debug => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    };
    let spec = format!(
        "{}, {} = {}",
        general_log_level,
        clap::crate_name!(),
        crate_log_level
    );
    Logger::try_with_str(&spec)?
        .format(detailed_format)
        .start()
        .with_context(|| "Error starting logger")?;

    debug!("{} version {}", clap::crate_name!(), clap::crate_version!());

    convert(
        matches.value_of("input-file"),
        matches.value_of("output-file"),
    )
}

fn make_input_reader(infile: Option<&str>) -> Result<io::BufReader<Box<dyn io::Read>>> {
    let inpath = infile.map(path::PathBuf::from);
    let reader = io::BufReader::new(match inpath {
        Some(path) => {
            debug!("Opening {} for read...", path.to_str().unwrap_or_default());
            Box::new(fs::File::open(path).with_context(|| "Error opening input file")?)
                as Box<dyn io::Read>
        }
        None => Box::new(io::stdin()) as Box<dyn io::Read>,
    });
    Ok(reader)
}

fn make_output_writer(outfile: Option<&str>) -> Result<io::BufWriter<Box<dyn io::Write>>> {
    let outpath = outfile.map(path::PathBuf::from);
    let writer = io::BufWriter::new(match outpath {
        Some(path) => {
            debug!("Opening {} for write...", path.to_str().unwrap_or_default());
            Box::new(fs::File::create(path).with_context(|| "Error opening output file")?)
                as Box<dyn io::Write>
        }
        None => Box::new(io::stdout()) as Box<dyn io::Write>,
    });
    Ok(writer)
}

fn convert(infile: Option<&str>, outfile: Option<&str>) -> Result<()> {
    let reader = make_input_reader(infile)?;
    let writer = make_output_writer(outfile)?;
    if let Some(inpath) = infile {
        debug!("Parsing {} as rtf.", inpath);
    } else {
        debug!("Parsing <stdin> as rtf.");
    }
    if let Some(outpath) = outfile {
        debug!("Writing parsed text to {}.", outpath);
    } else {
        debug!("Writing parsed text to <stdout>.");
    }
    let tokens = rtftotext::tokenize(reader)?;
    rtftotext::write_plaintext(&tokens, writer)
}
