pub mod txs;

use crate::ids;

/// ref. <https://pkg.go.dev/github.com/luxfi/node/utils/constants#pkg-variables>
pub fn chain_id() -> ids::Id {
    ids::Id::empty()
}
