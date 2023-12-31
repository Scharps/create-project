mod app;
mod prelude;
mod software;

use crate::app::Args;
use app::App;
use clap::Parser;

fn main() {
    let app: App = Args::parse().into();
    app.run();
    println!("Done.");
}
