use api::RepologyClient;

mod api;
mod os;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rep_client = RepologyClient::new();

    // Find the name of this project for this operating system and package manager
    let projects = rep_client.get_projects("curl").await?;
    dbg!(projects);

    // Use `project` for installation ..

    Ok(())
}
