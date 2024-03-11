import dgram from "dgram";

const argv = process.argv[2];

const PORT = 6969;

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

  client.send("connect", PORT, 'localhost');

  client.on('message', (message, remote) => {
    if (remote.address == '127.0.0.1' && remote.port == PORT) {
      const server_peers = JSON.parse(message);
      console.log(`got peer list from server`);
      console.debug(peers)

      peers = server_peers;

      peers.forEach(peer => {
        client.send("hey, I'm a peer, just like you!", peer.port, peer.address)
      })
      
      return;
    }

    console.log("got a peer message!");
    console.debug({ message: message.toString(), peer: `${remote.address}:${remote.port}` });
  })
} else {
  throw "expecting a --client or --server flag"
}
