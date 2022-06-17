use std::{env, fs, io};
use std::fs::File;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json;
use simple_logger::SimpleLogger;
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonResume {
    basics: Basics,
    work: Vec<Work>,
    education: Vec<Education>,
    skills: Vec<Skill>,
    projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Basics {
    name: String,
    label: String,
    image: String,
    email: String,
    phone: String,
    url: String,
    summary: String,
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    address: String,
    postal_code: String,
    city: String,
    country_code: String,
    region: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Work {
    name: String,
    position: String,
    start_date: String,
    end_date: Option<String>,
    summary: Option<String>,
    highlights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Education {
    institution: String,
    url: String,
    area: String,
    study_type: String,
    start_date: String,
    end_date: Option<String>,
    score: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Skill {
    name: String,
    keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    name: String,
    description: String,
    entity: String,
}

fn main() -> Result<(), io::Error> {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let output_filename = &args[2];

    info!("Reading ResumeJson file: {}", filename);
    let file = File::open(filename)?;
    let resume: JsonResume = serde_json::from_reader(file)?;
    
    let template_name = "resume.tex";
    let mut tera = Tera::default();

    match tera.add_raw_template(template_name, include_str!("templates/resume.template")) {
        Err(e) => {
            error!("Error adding template: {}", e);
            std::process::exit(1);
        }
        _ => {}
    };

    let context = match Context::from_serialize(&resume) {
        Ok(value) => value,
        Err(e) => {
            error!("Error serializing: {}", e);
            std::process::exit(1);
        }
    };

    let rendered = match tera.render(template_name, &context) {
        Ok(x) => x,
        Err(e) => {
            error!("Error rendering: {}", e);
            std::process::exit(1);
        }
    };

    match fs::write(output_filename, rendered) {
        Err(e) => {
            error!("Error writing: {}", e);
            std::process::exit(1);
        }
        _ => Ok(())
    }
}
