use std::env;
use todo_bin::{help, Todo};

fn main() {
    let todo= Todo::new().expect("Couldn't");

    let args: Vec<String>= env::args().collect();

    if args.len() > 1 {
        let command= &args[1];
        match &command[..]{
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            //....
        }
    }else {
        todo.list();
    }
}
