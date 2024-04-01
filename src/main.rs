mod app;

fn main() -> Result<(), ()> {
    println!("Hello, world!");

    app::App::new().start();

    println!("Bye, world!");
    Ok(())
}
