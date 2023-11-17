mod app;
use app::App;

fn main() {
    let app = App::new();
    app.run();
    println!("Done.");
}
