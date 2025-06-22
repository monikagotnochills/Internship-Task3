use std::fs::File;
use std::io::{self};
use printpdf::*;
use std::convert::From;

fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

fn assign_grade(average: f32) -> char {
    match average {
        a if a >= 90.0 => 'A',
        a if a >= 75.0 => 'B',
        a if a >= 60.0 => 'C',
        _ => 'D',
    }
}

fn generate_pdf(name: &str, total_marks: f32, num_subjects: u32, average: f32, grade: char) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap()).unwrap();

    let content = format!(
        "Student Report Card\n\nName: {}\nTotal Marks: {}\nSubjects: {}\nAverage: {:.2}\nGrade: {}",
        name, total_marks, num_subjects, average, grade
    );

    current_layer.use_text(content, 12.0, Mm(10.0), Mm(270.0), &font);

    let output = File::create("report_card.pdf").unwrap();
    doc.save(&mut io::BufWriter::new(output)).unwrap();
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();

    let name = name.trim();
    let total_marks: f32 = total_marks.trim().parse().unwrap();
    let num_subjects: u32 = num_subjects.trim().parse().unwrap();

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nGenerating report card PDF...");
    generate_pdf(name, total_marks, num_subjects, average, grade);
    println!("PDF saved as report_card.pdf");
}
