use std::path::{Path};

extern crate binary_reader;

mod tool_options;
mod zipper_formats;

pub use self::tool_options::{Options};
pub use self::zipper_formats::zrb::{ZrbHeader, ZRB_FOOTER};
pub use self::zipper_formats::clv::{ClvHeader, ClvEntry};
pub use self::zipper_formats::lips::{Lips, LipsEntry};

fn main() {
    // We use clap here in order to create the output
    let argument_matches = clap::App::new("Zipper file format tool")
        .version("0.0.1")
        .author("kd (@kd_tech_)")
        .about("Tool for interacting with zipper games (SOCOM, MAG) files")
        .arg(clap::Arg::with_name("input_file")
            .short("i")
            .long("inputFile")
            .takes_value(true)
            .required(true)
            .help("The input file"))
        .arg(clap::Arg::with_name("output_directory")
            .short("o")
            .long("outputDirectory")
            .takes_value(true)
            .help("Output file directory (default: current directory)"))
        .arg(clap::Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .takes_value(false)
            .help("Disables verbose logging"))
        .get_matches();
    
    // Input file name
    let input_file_path = argument_matches.value_of("input_file");
    if input_file_path.is_none() || !Path::new(input_file_path.unwrap()).exists() {
        eprintln!("err: invalid input file.");
        return;
    }

    // Get the output directory
    let output_directory_path = argument_matches.value_of("ouput_directory").unwrap_or(".");
    
    // Get if we have verbose output
    let quiet = argument_matches.value_of("quiet").unwrap_or("false").parse::<bool>().unwrap();

    // Create the options structure that will be passed around
    let options = tool_options::Options {
        input_file: String::from(input_file_path.unwrap()),
        output_directory: String::from(output_directory_path),
        quiet: quiet
    };

    // Check to see if we have verbose output enabled
    if !options.quiet {
        println!("input: ({})", options.input_file);
        println!("output: ({})", options.output_directory);
        println!("verbose: ({})", !options.quiet);
    }

    parse_input_file(&options);
}

/// Parses the file name and calls a extension handler if found
/// 
/// # Arguments
/// 
/// * `options` - A tool options configuration used for getting the input file path.
fn parse_input_file(options: &tool_options::Options) {
    let file_path = Path::new(&options.input_file);
    if !file_path.exists() {
        eprintln!("err: file ({}) does not exist.", options.input_file);
        return;
    }

    // We will load based on the file extension
    // TODO: Is there a better way than unwrap unwrap?
    let file_extension = file_path.extension().unwrap().to_str().unwrap();
    match &file_extension[..] {
        "zrb" => handle_zrb(options),
        "clv" => handle_clv(options),
        "lips" => handle_lips(options),
        _ => eprintln!("err: unknown file extension ({}).", file_extension)
    };
}

/// Opens up the provided zrb file for reading
/// 
/// # Arguments
/// 
/// * `_options` - A tool options configuration used for getting the input file path.
fn handle_zrb(_options: &tool_options::Options) {
    // Open up the file for reading
    let mut zrb_file = match std::fs::File::open(&_options.input_file) {
        Err(why) => panic!("could not open file {} for reading {}.", _options.input_file, why),
        Ok(file) => file
    };

    let mut zrb_reader = binary_reader::BinaryReader::from_file(&mut zrb_file);

    let zrb_header = zipper_formats::zrb::ZrbHeader::from_reader(&mut zrb_reader);

    println!("header: ");
    println!("unknown00: {}", zrb_header.unknown00);
}

fn handle_clv(options: &tool_options::Options) {
    // Open up the file for reading
    let mut clv_file = match std::fs::File::open(&options.input_file) {
        Err(why) => panic!("could not open file {} for reading {}.", options.input_file, why),
        Ok(file) => file
    };

    let mut clv_reader = binary_reader::BinaryReader::from_file(&mut clv_file);

    let clv_header = zipper_formats::clv::ClvHeader::from_reader(&mut clv_reader);

    println!("header: ");
    println!("magic: {}", clv_header.magic);
}

fn handle_lips(options: &tool_options::Options) {
    let lip = Lips::from_file(&options.input_file);

    println!("lips string count: ({}).", lip.string_count);

    println!("lips entry count: ({}).", lip.entries.len());
}