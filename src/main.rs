use open_api_to_http::app::Application;

fn main() {
    let app = Application::prepare().unwrap_or_else(|code| {
        std::process::exit(code);
    });

    app.run().unwrap_or_else(|code| {
        std::process::exit(code);
    });
}
