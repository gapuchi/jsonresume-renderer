pub mod schema {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JsonResume {
        basics: Basics,
        work: Option<Vec<Work>>,
        volunteer: Option<Vec<Volunteer>>,
        education: Option<Vec<Education>>,
        awards: Option<Vec<Award>>,
        certificates: Option<Vec<Certificate>>,
        publications: Option<Vec<Publication>>,
        skills: Option<Vec<Skill>>,
        languages: Option<Vec<Language>>,
        interests: Option<Vec<Interest>>,
        references: Option<Vec<Reference>>,
        projects: Option<Vec<Project>>,
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
        profiles: Option<Vec<Profile>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Profile {
        network: Option<String>,
        username: Option<String>,
        url: Option<String>,
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
        url: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        summary: Option<String>,
        highlights: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Volunteer {
        organization: Option<String>,
        position: Option<String>,
        url: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        summary: Option<String>,
        highlights: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Education {
        institution: Option<String>,
        url: Option<String>,
        area: Option<String>,
        study_type: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        score: Option<String>,
        courses: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Award {
        title: Option<String>,
        date: Option<String>,
        awarder: Option<String>,
        summary: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Certificate {
        name: Option<String>,
        date: Option<String>,
        issuer: Option<String>,
        url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Publication {
        name: Option<String>,
        publisher: Option<String>,
        release_date: Option<String>,
        url: Option<String>,
        summary: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Skill {
        name: Option<String>,
        level: Option<String>,
        keywords: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Language {
        language: Option<String>,
        fluency: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Interest {
        name: Option<String>,
        keywords: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Reference {
        name: Option<String>,
        reference: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Project {
        name: Option<String>,
        description: Option<String>,
        highlights: Option<Vec<String>>,
        keywords: Option<Vec<String>>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        url: Option<String>,
        roles: Option<Vec<String>>,
        entity: Option<String>,
        #[serde(rename = "type")]
        kind: Option<String>,
    }
}
