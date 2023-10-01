use clap::Parser;
use holochain_client::AppWebsocket;
use tokio;

/// Verify a holochain conductor is running and has a reachable app websocket at a given URL
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Holochain App WS URL
    app_ws: String,

    /// hApp app-id running on the conductor
    #[arg(short='i', long)]
    app_id: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut client: AppWebsocket = AppWebsocket::connect(args.app_ws.clone().into()).await
        .expect(format!("Failed to connect to app websocket at '{}'", args.app_ws.clone()).as_str());
    println!("Successfully connected to app websocket at {}", args.app_ws);

    if let Some(app_id) = args.app_id {
        let app_info = client.app_info(app_id.clone()).await
            .expect(format!("Failed to call client.app_info for app_id '{}'", app_id.clone()).as_str());
    
        println!("Received AppInfo: {:?}", app_info);
    }
}
