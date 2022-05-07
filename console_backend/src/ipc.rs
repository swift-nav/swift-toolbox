use ipc_channel::ipc;
use serde::{Serialize, Deserialize};

pub const ENV_PREFIX: &str = "SNAV_";

pub const IPC_SERVER_NAME: &str = "SNAV_SERVER";
pub const IPC_CLIENT_NAME: &str = "SNAV_CLIENT";


fn create_ipc_channel() -> (ipc::IpcSender<IpcMessage>, ipc::IpcReceiver<IpcMessage>) {
    ipc::channel().unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    Buffer(Vec<u8>),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientIpcConfig {
    pub server: String,
    pub client: String,
}

/// Called from the client, initializes to IPC channels, first is from serer -> client, second is
/// from client -> server.
pub fn init_ipc_channel() -> (ipc::IpcReceiver<IpcMessage>, ipc::IpcSender<IpcMessage>) {
    // Create the server->client IPC channel
    let (tx_client, rx_client) = create_ipc_channel();

    let ipc_config: ClientIpcConfig = envy::prefixed(ENV_PREFIX)
        .from_env()
        .expect("IPC information must be in the environment");

    // Send tx end of the channel to the server
    let oneshot_tx: ipc::IpcSender<ipc::IpcSender<IpcMessage>> =
        ipc::IpcSender::connect(ipc_config.server)
            .expect("connecting oneshot for server IPC must succeed");
    oneshot_tx
        .send(tx_client)
        .expect("sending server->client tx IPC endpoint must succeed");

    // Create the client->server IPC channel
    let (tx_server, rx_server) = create_ipc_channel();

    // Send rx end of the channel to the server
    let oneshot_tx: ipc::IpcSender<ipc::IpcReceiver<IpcMessage>> =
        ipc::IpcSender::connect(ipc_config.client)
            .expect("connect oneshot for client IPC must succeed");
    oneshot_tx
        .send(rx_server)
        .expect("sending client->server rx IPC endpoint must succeed");

    (rx_client, tx_server)
}