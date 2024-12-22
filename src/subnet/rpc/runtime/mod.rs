pub mod client;

use std::io::Result;

/// Runtime client dial timeout in seconds.
pub(crate) const DEFAULT_DIAL_TIMEOUT: u64 = 10;
/// Address of the VM runtime engine server.
pub(crate) const ENGINE_ADDR_KEY: &str = "LUX_VM_RUNTIME_ENGINE_ADDR";

/// ref. <https://github.com/luxfi/node/blob/master/vms/rpcchainvm/runtime/README.md>
#[tonic::async_trait]
pub trait Initializer {
    // Provides LuxGo with compatibility, networking and process
    // information of a VM.
    async fn initialize(&self, protocol_version: u32, vm_server_addr: &str) -> Result<()>;
}
