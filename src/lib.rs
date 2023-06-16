
use pyo3::prelude::*;
use std::fs::read_to_string;
use std::collections::HashMap;
use convert_case::{Case, Casing};

#[pymodule]
fn title_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    // read simple text file line by line in rust
    m.add_function(wrap_pyfunction!(parse_titles, m)?)?;
    Ok(())
}

#[pyfunction]
fn parse_titles(filename: &str) -> HashMap<Title, Vec<String>> {
    let file_lines = read_lines(filename);
    return find_titles(file_lines)
}

fn read_lines(filename: &str) -> Vec<String> {
    // Read the file contents into a vector of srings
    let result = read_to_string(filename);

    let contents = match result {
        Ok(contents) => contents,
        Err(error) => error.to_string()
    };

    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    return lines;
}



#[derive(Eq, Hash, PartialEq, Debug)]
struct Title(String);

impl IntoPy<PyObject> for Title {
    fn into_py(self, py: Python) -> PyObject {
        self.0.to_string().into_py(py)
    }
}

struct TitleLine(String);

impl TitleLine {
    // title lines are lines that start with ---- and end with ----
    // title lines contain the actual title in between the ----
    // titles lines can be parsed into a Title struct.
    fn is_title(&self) -> bool {
        return self.0.starts_with("----") && self.0.ends_with("----");
    }

    fn get_title(&self) -> Title {
        // use string trimming to remove the leading and trailing ----
        let trimmed = self.0.trim_start_matches("-").trim_end_matches("-");
        return Title(trimmed.to_string().to_case(Case::Snake));

    }

}

fn find_titles(search_lines: Vec<String>) -> HashMap<Title, Vec<String>> {
    // Create a dictionary of titles in the file to their content lines
    // The first lines without a title are assumed to be metadata.
    let mut current_title = Title("file_metadata".to_string());
    let mut contents: Vec<String> = Vec::new();
    let mut title_dict: HashMap<Title, Vec<String>> = HashMap::new();


    for current_line in &search_lines {
        if TitleLine(current_line.to_string()).is_title(){
            // flush contents and current title to title_dict
            title_dict.insert(current_title, contents.clone());

            // update current title and reset contents
            current_title = TitleLine(current_line.to_string()).get_title();
            contents.clear(); // wait this works? since rust guarantees no references, wow!
        }
        else {contents.push(current_line.to_string())}

    }

    if contents.len() > 0 {
        title_dict.insert(current_title, contents.clone());
    } else if !title_dict.contains_key(&current_title) {
        title_dict.insert(current_title, contents.clone()); // empty concluding title!
    }

    return title_dict;
}