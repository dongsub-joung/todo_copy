

pub struct Todo{
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String>{
        let todo_path: String= match env::var("TODO_PATH"){
            Ok(t) => t,
            Err(_) => {
                let home= evn::var("HOME").unwrap();
                
