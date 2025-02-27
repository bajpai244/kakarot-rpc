use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use jsonrpsee::server::RegisterMethodError;
use jsonrpsee::{Methods, RpcModule};

use crate::eth_provider::provider::EthereumProvider;
use crate::eth_rpc::api::alchemy_api::AlchemyApiServer;
use crate::eth_rpc::api::eth_api::EthApiServer;
use crate::eth_rpc::api::net_api::NetApiServer;
use crate::eth_rpc::api::web3_api::Web3ApiServer;
use crate::eth_rpc::servers::alchemy_rpc::AlchemyRpc;
use crate::eth_rpc::servers::eth_rpc::KakarotEthRpc;
use crate::eth_rpc::servers::net_rpc::NetRpc;
use crate::eth_rpc::servers::web3_rpc::Web3Rpc;

/// Represents RPC modules that are supported by reth
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KakarotRpcModule {
    Eth,
    Alchemy,
    Web3,
    Net,
}

pub struct KakarotRpcModuleBuilder<P>
where
    P: EthereumProvider + Send + Sync,
{
    modules: HashMap<KakarotRpcModule, Methods>,
    _phantom: PhantomData<P>,
}

impl<P> KakarotRpcModuleBuilder<P>
where
    P: EthereumProvider + Send + Sync + 'static,
{
    pub fn new(eth_provider: P) -> Self {
        let eth_provider = Arc::new(eth_provider);
        let eth_rpc_module = KakarotEthRpc::new(eth_provider.clone()).into_rpc();
        let alchemy_rpc_module = AlchemyRpc::new(eth_provider.clone()).into_rpc();
        let web3_rpc_module = Web3Rpc::default().into_rpc();
        let net_rpc_module = NetRpc::new(eth_provider).into_rpc();

        let mut modules: HashMap<KakarotRpcModule, Methods> = HashMap::new();

        modules.insert(KakarotRpcModule::Eth, eth_rpc_module.into());
        modules.insert(KakarotRpcModule::Alchemy, alchemy_rpc_module.into());
        modules.insert(KakarotRpcModule::Web3, web3_rpc_module.into());
        modules.insert(KakarotRpcModule::Net, net_rpc_module.into());

        Self { modules, _phantom: PhantomData }
    }

    pub fn rpc_module(&self) -> Result<RpcModule<()>, RegisterMethodError> {
        let mut rpc_module = RpcModule::new(());

        for methods in self.modules.values().cloned() {
            rpc_module.merge(methods)?;
        }

        Ok(rpc_module)
    }
}
