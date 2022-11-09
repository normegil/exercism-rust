// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    grades: HashMap<u32, Grade>
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade).or_insert_with(|| Grade::new()).add(student)
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grade: Vec<u32> = self.grades.keys().map(|k| *k).collect();
        grade.sort();
        grade
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            Option::None => vec![],
            Option::Some(grade) => {
                let mut names = grade.names.clone();
                names.sort();
                return names;
            },
        }
    }
}

struct Grade {
    names: Vec<String>
}

impl Grade {
    fn new() -> Self {
        Grade { names: Vec::new() }
    }

    fn add(&mut self, name: &str) {
        self.names.push(name.to_string())
    }
}
