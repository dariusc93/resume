use serde::{Deserialize, Serialize};

const ABOUT: &str = include_str!("../datafile/about.json");
const PROJECTS: &str = include_str!("../datafile/projects.json");
const RESUME: &str = include_str!("../datafile/resume.json");
const TERMS: &str = include_str!("../datafile/terms.json");
const PRIVACY: &str = include_str!("../datafile/privacy.json");

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub title: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

impl Profile {
    pub fn show_get_in_touch(&self) -> bool {
        self.email.is_some() || self.linkedin.is_some() || self.github.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Experience {
    pub position: String,
    pub company: String,
    pub duration: String,
    pub description: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub school: String,
    pub year: Option<String>,
    pub details: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillCategory {
    pub name: String,
    pub items: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resume {
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<SkillCategory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub image: Option<String>,
    pub date: Option<String>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub message: String,
}


pub fn get_resume() -> Resume {
    serde_json::from_str(RESUME).expect("valid json")
}

pub fn get_projects() -> Vec<Project> {
    let mut projects: Vec<Project> = serde_json::from_str(PROJECTS).expect("valid json");
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    projects
}

pub fn get_about() -> About {
    serde_json::from_str(ABOUT).expect("valid json")
}

pub fn get_terms() -> Terms {
    serde_json::from_str(TERMS).expect("valid json")
}

pub fn get_privacy() -> Privacy {
    serde_json::from_str(PRIVACY).expect("valid json")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AboutSection {
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct About {
    pub intro: String,
    pub sections: Vec<AboutSection>,
    pub interests: Vec<String>,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegalSection {
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Terms {
    pub title: String,
    pub effective_date: String,
    pub sections: Vec<LegalSection>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Privacy {
    pub title: String,
    pub effective_date: String,
    pub sections: Vec<LegalSection>,
}
