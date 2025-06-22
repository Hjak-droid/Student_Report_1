use std::fs::File;
use std::io::{BufWriter, Write};
use printpdf::*;

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(average: f32) -> &'static str {
    if average >= 90.0 {
        "A"
    } else if average >= 75.0 {
        "B"
    } else if average >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn generate_pdf_report(name: &str, total_marks: f32, subjects: u32, average: f32, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Student Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let text = format!(
        "Student Report Card\n\nName: {}\nTotal Marks: {}\nSubjects: {}\nAverage: {:.2}\nGrade: {}",
        name, total_marks, subjects, average, grade
    );

    let font = doc
    .add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap())
    .unwrap();


    current_layer.use_text(text, 14.0, Mm(10.0), Mm(270.0), &font);

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap()))
        .unwrap();
}

fn main() {
    let name = String::from("Tilak");
    let total_marks = 450.0;
    let subjects = 5;

    let average = calculate_average(total_marks, subjects);
    let grade = assign_grade(average);

    println!("--- Student Report ---");
    println!("Name: {}", name);
    println!("Total Marks: {}", total_marks);
    println!("Subjects: {}", subjects);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);

    generate_pdf_report(&name, total_marks, subjects, average, grade);

    println!("PDF report generated successfully!");
}
