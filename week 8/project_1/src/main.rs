#[derive(Debug)]
enum JobTitle {
    Ceo,
    Director,
    SeniorLecturer,
    Dean,
    Partner,
    Principal,
    OfficeManager,
    Researcher,
    PostDoctoral,
    SeniorAssociate3,
    LeadingTeacher,
    SeniorAdministrator,
    PhdCandidate,
    Associate,
    SeniorAssociate1,
    SeniorTeacher,
    Administrator,
    ResearchAssistant,
    JuniorAssociate,
    ClassroomTeacher,
    Intern,
    OfficeAdministrator,
    Placement,
    Paralegal,
    Teacher,
}

#[derive(Debug)]
struct Job {
    title: JobTitle,
    years_of_experience: i32,
    aps_level: String,
}

impl Job {
    fn new(title: JobTitle, years_of_experience: i32) -> Self {
        let aps_level = match title {
            JobTitle::Ceo | JobTitle::Director | JobTitle::SeniorLecturer | JobTitle::Dean | JobTitle::Partner | JobTitle::Principal => {
                if years_of_experience >= 30 {
                    "SES".to_string()
                } else {
                    "EL2 10-13".to_string()
                }
            }
            JobTitle::OfficeManager | JobTitle::Researcher | JobTitle::PostDoctoral | JobTitle::SeniorAssociate3 | JobTitle::LeadingTeacher => {
                if years_of_experience >= 30 {
                    "EL1 8-10".to_string()
                } else {
                    "APS 5-8".to_string()
                }
            }
            JobTitle::SeniorAdministrator | JobTitle::PhdCandidate | JobTitle::Associate | JobTitle::SeniorAssociate1 | JobTitle::SeniorTeacher => {
                if years_of_experience >= 30 {
                    "APS 5-8".to_string()
                } else {
                    "APS 1-2".to_string()
                }
            }
            JobTitle::Administrator | JobTitle::ResearchAssistant | JobTitle::JuniorAssociate | JobTitle::ClassroomTeacher => "APS 1-2".to_string(),
            JobTitle::Intern => "Public Servant".to_string(),
            JobTitle::OfficeAdministrator => "Office Administrator".to_string(),
            JobTitle::Placement => "Academic".to_string(),
            JobTitle::Paralegal => "Lawyer".to_string(),
            JobTitle::Teacher => "Teacher".to_string(),
        };
        
        Job {
            title,
            years_of_experience,
            aps_level,
        }
    }
}

fn main() {
    let job = Job::new(JobTitle::SeniorLecturer, 30);
    println!("{:?}", job);
}