use std::error::Error;
use std::{fs, io, path};

use log::{debug, error};

mod error;
mod rtf_control;
mod rtftotext;

fn main() {
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

    loggerv::init_with_verbosity(matches.occurrences_of("debug")).unwrap();

    debug!("{} version {}", clap::crate_name!(), clap::crate_version!());

    if let Err(e) = convert(
        matches.value_of("input-file"),
        matches.value_of("output-file"),
    ) {
        eprintln!("ERROR: {}", e);
        if let Some(inner) = e.source() {
            eprintln!("Cause: {}", inner);
        }
        std::process::exit(e.code());
    }
}

fn make_input_reader(infile: Option<&str>) -> error::Result<io::BufReader<Box<dyn io::Read>>> {
    let inpath = infile.map(path::PathBuf::from);
    let reader = io::BufReader::new(match inpath {
        Some(path) => {
            debug!("Opening {} for read...", path.to_str().unwrap_or(""));
            let file = fs::File::open(path.clone());
            if let Err(e) = file {
                error!("ERROR: {}", e);
                return Err(error::Error::from_input_error(e));
            } else {
                Box::new(fs::File::open(path).map_err(error::Error::from_input_error)?)
                    as Box<dyn io::Read>
            }
        }
        None => Box::new(io::stdin()) as Box<dyn io::Read>,
    });
    Ok(reader)
}

fn make_output_writer(outfile: Option<&str>) -> error::Result<io::BufWriter<Box<dyn io::Write>>> {
    let outpath = outfile.map(path::PathBuf::from);
    let writer = io::BufWriter::new(match outpath {
        Some(path) => {
            debug!("Opening {} for write...", path.to_str().unwrap_or(""));
            Box::new(fs::File::create(path).map_err(error::Error::from_output_error)?)
                as Box<dyn io::Write>
        }
        None => Box::new(io::stdout()) as Box<dyn io::Write>,
    });
    Ok(writer)
}

fn convert(infile: Option<&str>, outfile: Option<&str>) -> error::Result<()> {
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
