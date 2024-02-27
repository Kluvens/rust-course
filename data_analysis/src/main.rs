const ENROLMENTS_PATH: &str = "enrolments.psv";
use std::{collections::{HashMap, HashSet}, io::{stdin, stdout, Write}};
use csv::{ReaderBuilder};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Row {
    course_code: String,
    student_id: u32,
    name: String,
    program: String,
    plan: String,
    wam: f64,
    session: String,
    bday: String,
    sex: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)?;

    let mut students = HashSet::new();
    let mut course_students: HashMap<String, HashSet<u32>> = HashMap::new();
    let mut num_students = 0;
    let mut wams = Vec::new();
    
    for result in rdr.deserialize() {
        let row: Row = result?;
        students.insert(row.student_id);
        wams.push(row.wam);
        course_students.entry(row.course_code.clone())
            .or_insert_with(HashSet::new)
            .insert(row.student_id);
    }

    num_students = students.len();

    let avg_wam = wams.iter().sum::<f64>() / wams.len() as f64;

    let (most_common_course, most_students) = course_students.iter()
        .max_by_key(|entry| entry.1.len())
        .map(|(course, students)| (course, students.len()))
        .unwrap();

    let (least_common_course, least_students) = course_students.iter()
        .min_by_key(|entry| entry.1.len())
        .map(|(course, students)| (course, students.len()))
        .unwrap();

    println!("Number of students: {}", num_students);
    println!("Most common course: {} with {} students", most_common_course, most_students);
    println!("Least common course: {} with {} students", least_common_course, least_students);
    println!("Average WAM of all students: {:.2}", avg_wam);

    Ok(())
}
