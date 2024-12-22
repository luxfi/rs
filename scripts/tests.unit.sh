#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ scripts/tests.unit.sh ]]; then
  echo "must be run from repository root"
  exit 255
fi

# TODO: better way to skip proto tests?
RUST_LOG=debug cargo test --workspace \
--features lux-types/node \
--features lux-types/codec_base64 \
--features lux-types/codec_big_int \
--features lux-types/evm \
--features lux-types/jsonrpc_client \
--features lux-types/kms_aws \
--features lux-types/libsecp256k1 \
--features lux-types/message \
--features lux-types/mnemonic \
--features lux-types/subnet \
--features lux-types/subnet_evm \
--features lux-types/wallet \
--features lux-types/wallet_evm \
--features lux-types/xsvm \
-- --show-output

echo "ALL SUCCESS!"
