pub mod schema {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JsonResume {
        basics: Basics,
        work: Vec<Work>,
        education: Vec<Education>,
        skills: Vec<Skill>,
        projects: Vec<Project>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Basics {
        name: Option<String>,
        label: Option<String>,
        image: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        url: Option<String>,
        summary: Option<String>,
        location: Location,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Location {
        address: Option<String>,
        postal_code: Option<String>,
        city: Option<String>,
        country_code: Option<String>,
        region: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Work {
        name: Option<String>,
        position: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        summary: Option<String>,
        highlights: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Education {
        institution: Option<String>,
        url: Option<String>,
        area: Option<String>,
        study_type: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        score: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Skill {
        name: Option<String>,
        keywords: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Project {
        name: Option<String>,
        description: Option<String>,
        entity: Option<String>,
    }
}
