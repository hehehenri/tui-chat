import dgram from "dgram";

const argv = process.argv[2];

const SIGNALING_ADDR = "18.229.163.193"; 
const SIGNALING_PORT = 6969;
const SIGNALING_URL = `${SIGNALING_ADDR}:${SIGNALING_PORT}`;

const isSignaling = peer => {
  return (
    peer.address === SIGNALING_ADDR &&
    peer.port === SIGNALING_PORT
  );
}

if (argv == "--server") {
  const server = dgram.createSocket('udp4');
  let peers = [];

  server.on('listening', () => {
    const address = server.address();
    console.log(`UDP Server listening on ${address.address}:${address.port}`);
  });

  server.on('message', (_, remote) => {
    console.log(`client connecting: ${remote.address}:${remote.port}`);
    const client = { address: remote.address, port: remote.port };
    peers.push(client);

    peers.forEach(peer => {  
      console.log({ peer, peers });
      const message = peers.filter(a => a != peer);

      server.send(JSON.stringify(message), peer.port, remote.address);
    });
  });

  server.bind(PORT);
} else if (argv == "--client") {
  let peers = [];
  
  const client = dgram.createSocket('udp4');

  client.send("connect", SIGNALING_PORT, SIGNALING_ADDR);

  client.on('message', (message, remote) => {
    const remotePeer = { port: remote.port, address: remote.address };
    
    if (isSignaling(remotePeer)) {
      const server_peers = JSON.parse(message);
      console.log(`INFO: got peer list from server`);
      console.debug(server_peers);

      peers = server_peers;

      peers.forEach(peer => {
        console.log(`INFO: sending message to ${peer.address}:${peer.port}"`);
        client.send("hey, I'm a peer, just like you!", peer.port, peer.address)
      })
      
      return;
    }

    console.log("INFO: got a peer message!");
    console.debug({ message: message.toString(), peer: `${remote.address}:${remote.port}` });
  })
} else {
  throw "expecting a --client or --server flag"
}
