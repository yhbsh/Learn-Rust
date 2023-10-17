const COURSES: [&str; 7] = ["C", "C++", "C#", "Java", "JavaScript", "Python", "PHP"];

use rand::Rng;
use std::collections::LinkedList;
use std::fs;
use std::io::Write;

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    first_name: String,
    last_name: String,
    average: u8,
}

fn main() {
    // get file path as argument to the program
    let file_path = std::env::args().nth(1).expect("Please provide a file path");
    let content = fs::read_to_string(file_path)
        .or_else(|err| -> Result<String, ()> {
            println!("ERROR: could not read file {err}");
            std::process::exit(1);
        })
        .unwrap();

    let lines: Vec<&str> = content.split("\n").collect();

    let mut current_index: usize = 0;
    let mut courses: Vec<LinkedList<Student>> = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Formation:") {
            // insert as head of current linked list
            let list: LinkedList<Student> = LinkedList::new();

            courses.push(list);
            current_index += 1;
        } else {
            // insert to the current linked list
            let current_course: &mut LinkedList<Student> = courses
                .get_mut(current_index - 1)
                .expect("ERROR: could not get current course");

            let first_name = line.split_whitespace().next().unwrap();
            let last_name = line.split_whitespace().last().unwrap();
            let average = rand::thread_rng().gen_range(7..20);
            let student = Student {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                average,
            };
            current_course.push_back(student);
        }
    }

    let mut choice = String::new();

    loop {
        // get user input
        println!("");
        println!("Please enter a command:");
        println!("");
        println!("0. Exit");
        println!("1. Traversals");
        println!("");

        std::io::stdin().read_line(&mut choice).unwrap();

        choice.pop();

        match choice.as_str().trim() {
            "0" => {
                std::process::exit(0);
            }
            "1" => loop {
                println!("");
                println!("0. go back");
                println!("1. get average of a student");
                println!("2. get the total average students in a course");
                println!("");

                let mut traversal_choice = String::new();
                std::io::stdin().read_line(&mut traversal_choice).unwrap();

                match traversal_choice.as_str().trim() {
                    "0" => {
                        break;
                    }
                    "1" => {
                        println!("");
                        print!("Enter student name: ");
                        std::io::stdout().flush().unwrap();

                        let mut student_name = String::new();
                        std::io::stdin().read_line(&mut student_name).unwrap();
                        student_name = student_name.trim().to_string();

                        println!("");

                        let mut found = false;
                        for course in &courses {
                            for student in course {
                                if student.first_name == student_name
                                    || student.last_name == student_name
                                {
                                    println!("Average: {}", student.average);
                                    found = true;
                                }
                            }
                        }

                        match found {
                            false => {
                                println!("Student not found");
                            }
                            _ => {}
                        }
                    }
                    "2" => {
                        let mut course_index = String::new();
                        println!("");

                        for i in 0..courses.len() {
                            println!("{}. {}", i + 1, COURSES[i]);
                        }

                        println!("");

                        print!("Enter course index: ");
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut course_index).unwrap();

                        let course_index: usize = course_index.trim().parse().unwrap();
                        let course = courses.get(course_index - 1).unwrap_or_else(
                            || -> &LinkedList<Student> {
                                println!("Course not found");
                                std::process::exit(1);
                            },
                        );

                        let mut total_average: u32 = 0;

                        for student in course {
                            total_average += student.average as u32;
                        }

                        let total_average = total_average / course.len() as u32;

                        println!("");
                        println!("Total average: {}", total_average);
                    }
                    _ => {
                        println!("Invalid command")
                    }
                }

                traversal_choice.clear();
                choice.clear();
            },
            _ => {
                println!("Invalid command");
            }
        }
    }
}
