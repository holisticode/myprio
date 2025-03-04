use clap::Parser;
use myprio::app::{App, Datasources};
use myprio::task::manager::TaskManager;

fn main() {
    env_logger::init();
    let app = App::parse();
    //let path = &app.path;
    let default_settings = myprio::app::default_sqllite_settings();
    let path = &default_settings.path;
    let ds = match app.datasource {
        None => Datasources::SqlLite,
        Some(ds) => ds,
    };

    let mgr = &mut TaskManager::new(ds, path);

    app.run_prompt(mgr);
}
