use env_logger;

use clap::Parser;
use fabtask::app::{App, Datasources};
use fabtask::task::manager::TaskManager;

fn main() {
    env_logger::init();
    let app = App::parse();
    //let path = &app.path;
    let default_settings = fabtask::app::default_sqllite_settings();
    let path = &default_settings.path;
    let ds = match app.datasource {
        None => Datasources::SqlLite,
        Some(ds) => ds,
    };

    let mgr = &mut TaskManager::new(ds, path);

    app.run_prompt(mgr);
}
