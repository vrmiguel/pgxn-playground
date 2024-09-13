use api::RepologyClient;
use os::{install_command, OperatingSystem};

mod api;
mod os;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dbg!(install_command("ripgrep", OperatingSystem::Debian).await?);
    dbg!(install_command("ripgrep", OperatingSystem::RedHat).await?);
    dbg!(install_command("ripgrep", OperatingSystem::Mac).await?);

    Ok(())
}
