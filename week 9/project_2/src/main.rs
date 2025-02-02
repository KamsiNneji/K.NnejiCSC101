use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: i32,
    marks: i32,
}

fn main() -> io::Result<()> {
    let students = vec![
        Student {
            name: "John".to_string(),
            matric_number: "2022001".to_string(),
            department: "Accounting".to_string(),
            level: 100,
            marks: 300,
        },
        Student {
            name: "Juliet".to_string(),
            matric_number: "2022002".to_string(),
            department: "Economics".to_string(),
            level: 200,
            marks: 320,
        },
        Student {
            name: "Sadiq".to_string(),
            matric_number: "2022003".to_string(),
            department: "Chemistry".to_string(),
            level: 300,
            marks: 310,
        },
        Student {
            name: "Bala".to_string(),
            matric_number: "2022004".to_string(),
            department: "Mechanical".to_string(),
            level: 400,
            marks: 300,
        },
    ];

    // Display student details
    for student in &students {
        println!(
            "Name: {}, Matric Number: {}, Department: {}, Level: {}, Marks: {}",
            student.name, student.matric_number, student.department, student.level, student.marks
        );
    }

    // Save student details to a file
    let mut file = File::create("student_details.txt")?;
    writeln!(file, "Student Name\tMatric Number\tDepartment\tLevel\tMarks")?;
    for student in &students {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}",
            student.name, student.matric_number, student.department, student.level, student.marks
        )?;
    }

    println!("Student details saved to 'student_details.txt'");

    Ok(())
}