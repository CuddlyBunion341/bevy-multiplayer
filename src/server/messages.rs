use renet::ClientId;

pub enum RenetServerMessage {
    PlayerConnected(ClientId),
    PlayerDisconnected(ClientId),
}
