use libp2p::{
    PeerId, identity,
    swarm::SwarmBuilder,
    mdns::Mdns,
    gossipsub::{self, MessageAuthenticity, GossipsubConfigBuilder},
    development_transport
};
use tokio::sync::mpsc;

pub async fn start_p2p() {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer ID: {:?}", local_peer_id);

    let transport = development_transport(local_key.clone()).await.unwrap();
    let gossipsub = gossipsub::Gossipsub::new(
        MessageAuthenticity::Signed(local_key),
        GossipsubConfigBuilder::default().build().unwrap()
    ).unwrap();
    
    let mdns = Mdns::new().await.unwrap();
    let swarm = SwarmBuilder::new(transport, gossipsub, local_peer_id.clone())
        .with_mdns(mdns)
        .build();

    println!("P2P Node started");
}
