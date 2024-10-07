use std::collections::{HashMap, HashSet};

pub struct School {
    grades: HashMap<u32, HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.is_student_in_any_grade(student) {
            self.grades
                .entry(grade)
                .or_insert_with(HashSet::new)
                .insert(student.to_string());
        }
    }

    fn is_student_in_any_grade(&self, student: &str) -> bool {
        self.grades
            .values()
            .any(|students_set| students_set.contains(student))
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = self.grades.keys().cloned().collect();
        vec.sort();
        vec
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            Some(set) => {
                let mut vec: Vec<String> = set.iter().cloned().collect();
                vec.sort();
                vec
            }
            None => vec![],
        }
    }
}
