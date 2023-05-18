use driver::module::Modules;
use driver::startup::{init_app, startup};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(modules).await;

    Ok(())
}
