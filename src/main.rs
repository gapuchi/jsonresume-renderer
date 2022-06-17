use std::{env, error, fs};
use std::fs::File;
use log::info;
use jsonresume_renderer::schema::JsonResume;
use serde_json;
use simple_logger::SimpleLogger;
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn error::Error>> {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    let json_resume_filename = &args[1];
    let template_filename = &args[2];
    let output_filename = &args[3];

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
