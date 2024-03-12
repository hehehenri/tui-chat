use crate::message::Join;
use crate::server::Context;
use crate::transport::Transport;

pub async fn handle(Join(new_peer): Join, transport: &Box<dyn Transport>, ctx: &Context) {
    match ctx.repositories.peer_repository.store(&new_peer).await {
        Ok(()) => println!("INFO: new peer added to the list of connections"),
        Err(err) => {
            eprintln!("ERROR: {}", err.to_string());
            return;
        }
    }

    let peers = match ctx.repositories.peer_repository.all().await {
        Ok(peers) => peers,
        Err(err) => {
            eprintln!("ERROR: {}", err.to_string());
            return;
        }
    };

    for peer in peers.iter() {
        match serde_json::to_string(&peers) {
            Ok(message) => {
                let response = transport.send_to(message.into_bytes(), peer.addr).await;
                match response {
                    Ok(()) => println!("INFO: list of peers sent to {}", peer.addr),
                    Err(err) => {
                        eprintln!(
                            "ERROR: failed to send the list of peers to {}: {}",
                            peer.addr,
                            err.to_string()
                        )
                    }
                }
            }
            Err(err) => eprintln!(
                "ERROR: failed to update {} peers: {}",
                peer.addr,
                err.to_string()
            ),
        }
    }
}
