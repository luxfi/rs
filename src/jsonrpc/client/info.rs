use std::{collections::HashMap, time::Duration};

use crate::{
    errors::{Error, Result},
    ids,
    jsonrpc::{self, info},
    utils,
};
use reqwest::{header::CONTENT_TYPE, ClientBuilder};

/// e.g., "info.getNetworkName".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetnetworkname>
pub async fn get_network_name(http_rpc: &str) -> Result<info::GetNetworkNameResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting network name for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkName");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.getNetworkID".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetnetworkid>
pub async fn get_network_id(http_rpc: &str) -> Result<info::GetNetworkIdResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting network Id for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkID");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.getBlockchainID".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetblockchainid>
pub async fn get_blockchain_id(
    http_rpc: &str,
    chain_alias: &str,
) -> Result<info::GetBlockchainIdResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting blockchain Id for {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("info.getBlockchainID");

    let mut params = HashMap::new();
    params.insert(String::from("alias"), String::from(chain_alias));
    data.params = Some(params);
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.getNodeID".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetnodeid>
pub async fn get_node_id(http_rpc: &str) -> Result<info::GetNodeIdResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting node Id for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeID");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    let resp: info::GetNodeIdResponse = serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })?;

    if let Some(res) = &resp.result {
        if let Some(pop) = &res.node_pop {
            let pubkey = pop.load_pubkey().map_err(|e| Error::Other {
                message: format!("failed pop.load_pubkey '{}'", e),
                retryable: false,
            })?;

            let mut cloned_pop = pop.clone();
            cloned_pop.pubkey = Some(pubkey);

            let mut cloned_result = res.clone();
            cloned_result.node_pop = Some(cloned_pop);

            let mut cloned_resp = resp.clone();
            cloned_resp.result = Some(cloned_result);

            Ok(cloned_resp)
        } else {
            return Err(Error::Other {
                message: "no result.node_pop found".to_string(),
                retryable: false,
            });
        }
    } else {
        return Err(Error::Other {
            message: "no result found".to_string(),
            retryable: false,
        });
    }
}

/// e.g., "info.getNodeVersion".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetnodeversion>
pub async fn get_node_version(http_rpc: &str) -> Result<info::GetNodeVersionResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting node version for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeVersion");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.getVMs".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogetvms>
pub async fn get_vms(http_rpc: &str) -> Result<info::GetVmsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting VMs for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getVMs");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.isBootstrapped".
/// ref. <https://docs.lux.network/build/node-apis/info/#infoisbootstrapped>
pub async fn is_bootstrapped(http_rpc: &str) -> Result<info::IsBootstrappedResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting bootstrapped for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.isBootstrapped");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.getTxFee".
/// ref. <https://docs.lux.network/build/node-apis/info/#infogettxfee>
/// ref. "genesi/genesis_mainnet.go" requires 1 * units::LUX for create_subnet_tx_fee/create_blockchain_tx_fee
/// ref. "genesi/genesis_fuji/local.go" requires 100 * units::MILLI_LUX for create_subnet_tx_fee/create_blockchain_tx_fee
pub async fn get_tx_fee(http_rpc: &str) -> Result<info::GetTxFeeResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting tx fee for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getTxFee");
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}

/// e.g., "info.peers".
/// ref. <https://docs.lux.network/build/node-apis/info/#infopeers>
pub async fn peers(
    http_rpc: &str,
    node_ids: Option<Vec<ids::node::Id>>,
) -> Result<info::PeersResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting peers for {u}");

    let mut data = jsonrpc::RequestWithParamsHashMapToArray::default();
    data.method = String::from("info.peers");
    let mut ids = Vec::new();
    if let Some(ss) = &node_ids {
        for id in ss.iter() {
            ids.push(id.to_string());
        }
    }
    let mut params = HashMap::new();
    params.insert(String::from("nodeIDs"), ids);
    data.params = Some(params);
    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
            // TODO: check retryable
            Error::API {
                message: format!("failed reqwest::Client.send '{}'", e),
                retryable: false,
            })?;
    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| Error::Other {
        message: format!("failed serde_json::from_slice '{}'", e),
        retryable: false,
    })
}
