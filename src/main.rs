use std::{error, fs};
use std::fs::File;
use log::info;
use jsonresume_renderer::schema::JsonResume;
use serde_json;
use simple_logger::SimpleLogger;
use tera::{Context, Tera};
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///Location of the JSON file following the JSON Resume schema.
    #[clap(long, short, value_parser)]
    json_resume_filename: String,

    ///Location of the Tera-compatible template file.
    #[clap(long, short, value_parser)]
    template_filename: String,

    ///Name of the file to write the results to.
    #[clap(long, short, value_parser)]
    output_filename: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    SimpleLogger::new().init().unwrap();

    let cli = Cli::parse();
    let json_resume_filename = cli.json_resume_filename;
    let template_filename = cli.template_filename;
    let output_filename = cli.output_filename;

    info!("Reading ResumeJson file: {}", json_resume_filename);
    let resume: JsonResume = serde_json::from_reader(File::open(json_resume_filename)?)?;

    info!("Reading template file: {}", template_filename);
    let template = fs::read_to_string(template_filename)?;

    info!("Generating Tera Context");
    let context = Context::from_serialize(&resume)?;

    info!("Rendering from template");
    let rendered = Tera::one_off(&template, &context, false)?;

    info!("Writing to file {}", output_filename);
    fs::write(output_filename, rendered)?;

    Ok(())
}
