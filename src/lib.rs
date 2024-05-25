use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub text: String,
    pub checked: bool,
    pub editing: bool,
    pub children:Vec<Todo>
}

// save todos to path
pub fn save(path: String, todos: &mut Vec<Todo>){
    let serialized:String = ron::ser::to_string(todos).unwrap().to_string();
    let file = File::create(path);
    file.unwrap().write_all(serialized.as_ref()).unwrap();
}

// open todos from path
pub fn open(path: String) -> Vec<Todo> {
    let file=File::open(path.clone());
    let mut contents=String::new();
    file.unwrap().read_to_string(&mut contents).expect(format!("File opening error! Does the file {} exist?", path).as_str());
    let todos:Vec<Todo> = ron::from_str(&contents as &str).expect("There seemed to be an error");
    todos
}