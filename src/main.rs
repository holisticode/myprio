use clap::Parser;
use fabtask::app::App;
use fabtask::task::manager::TaskManager;

fn main() {
    let app = App::parse();
    let path = app.path.as_ref().unwrap();
    let mgr = &mut TaskManager::new(&app.datasource, path);

    app.run_prompt(mgr);
}
