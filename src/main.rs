use sigmaos_first_boot::{Config, FirstBootComponent, Component};
fn main() {
    env_logger::init();
    let mut c = FirstBootComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS first-boot running");
    let _ = c.shutdown();
}
