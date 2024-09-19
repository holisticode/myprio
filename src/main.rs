mod task;

fn main() {
    let mut mgr = task::manager::TaskManager::new();
    match mgr.add(String::from("first"), String::from("first test task")) {
        Ok(_) => println!("it worked"),
        Err(_) => println!("failed"),
    }
}
