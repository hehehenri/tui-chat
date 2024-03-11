use crate::{message::Join, server::Context, transport::Transport};

pub fn handle(Join(new_peer): Join, ctx: &Context, transport: &Box<dyn Transport>) {
    let mut peers = ctx.peers.lock().unwrap();
    peers.push(new_peer);

    let peers = peers.clone();

    peers.iter().for_each(|peer| {
        match serde_json::to_string(&peers) {
            Ok(message) => {
                let response = transport.send_to(message.into_bytes(), peer.addr);
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
        };
    });
}
