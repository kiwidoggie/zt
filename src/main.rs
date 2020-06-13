use std::path::{Path};

mod zipper_tool;

extern crate binary_reader;

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
    let options = zipper_tool::Options {
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

fn handle_zrb(_options: &Options) {
    let mut _header = ZrbHeader {
        unknown00: 0
    };

    _header.parse(_options);
}

fn parse_input_file(options: &Options) {
    //let file_path = Path::new(&options.input_file);
    //let file_extension = file_path.extension();

    // TODO: Figure out what the fuck
    handle_zrb(options);
}
