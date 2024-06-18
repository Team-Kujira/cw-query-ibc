# Kujira Bindings

CosmWasm bindings to custom Kujira features.
This repo provides binding contracts to query verification result for membership or absense of merkle proof through Kujira core.

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## Build

```
cargo build --target wasm32-unknown-unknown --release
```

## Preparing the Wasm bytecode for production

Before we upload it to a chain, we need to ensure the smallest output size possible,
as this will be included in the body of a transaction. We also want to have a
reproducible build process, so third parties can verify that the uploaded Wasm
code did indeed come from the claimed rust code.

To solve both these issues, we have produced `rust-optimizer`, a docker image to
produce an extremely small build output in a consistent manner. The suggest way
to run it is this:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.13
```

Or, If you're on an arm64 machine, you should use a docker image built with arm64.

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.12.13
```

This produces an `artifacts` directory with a `PROJECT_NAME.wasm`, as well as
`checksums.txt`, containing the Sha256 hash of the wasm file.

## Interact with Kujira through CLI

### Setup IBC connected chains locally

1. Clone the [Kujira core](https://github.com/antstalepresh/core/tree/ibc_wasm_binding) and install the daemon using `make install`.
2. For counterparty chain, clone the [Terra core](https://github.com/terra-money/core/tree/v2.4.1) and install the daemon using `make install`.
3. For relayer, clone the [Go relayer](https://github.com/cosmos/relayer/tree/v2.4.2) and install the daemon using `make install`.
4. Once both are installed, run the following script for setting up IBC env automatically.

```
bash test/setup_ibc.sh
```

It should end up like this.

```
...
2023-10-12T10:05:22.866532Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 167530, "fees": "3879uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 8, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenTry"], "tx_hash": "21F0A9C6D938247CB35082F9240CCA8A44755479D2BD933B9BFFD0154BDFB550"}
2023-10-12T10:05:32.450954Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 146348, "fees": "3328ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 11, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenAck"], "tx_hash": "F9B7E8C8387FFEE8D9106CFD643A9DCD0884F1B5CE48A748A82655FDA2EC2151"}
2023-10-12T10:05:38.223788Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 130011, "fees": "2903uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 11, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenConfirm"], "tx_hash": "AD9DF7A3C3B85E1CDBB9D286F7C58AAB05E47F795E7C4A21FB553D58529D5F2A"}
2023-10-12T10:05:38.488771Z	info	Connection handshake termination candidate	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0", "termination_client_id": "07-tendermint-0", "observed_client_id": "07-tendermint-0", "termination_counterparty_client_id": "07-tendermint-0", "observed_counterparty_client_id": "07-tendermint-0"}
2023-10-12T10:05:38.488790Z	info	Found termination condition for connection handshake	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0"}
2023-10-12T10:05:38.498030Z	info	Starting event processor for channel handshake	{"src_chain_id": "kujira", "src_port_id": "transfer", "dst_chain_id": "terra", "dst_port_id": "transfer"}
2023-10-12T10:05:38.499847Z	info	Chain is in sync	{"chain_name": "terra", "chain_id": "terra"}
2023-10-12T10:05:38.499908Z	info	Chain is in sync	{"chain_name": "kujira", "chain_id": "kujira"}
2023-10-12T10:05:47.783022Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 161164, "fees": "3713ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 14, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenInit"], "tx_hash": "4941B6EAD5431DD7DC66C154ADDD04C1A648B38AEA9D00F8D1448F9F462D9595"}
2023-10-12T10:05:58.583442Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 184197, "fees": "4312uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 15, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenTry"], "tx_hash": "1300BED1252972C5ECB246607B244EDF1E53A53E2FA6C328608D5982F5C66D62"}
2023-10-12T10:06:08.171426Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 123216, "fees": "2727ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 18, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenAck"], "tx_hash": "43D9F7DCC770A1D4C38EDCE4A226808C1115950E0A6929FFE6A485D6DF4E3403"}
2023-10-12T10:06:08.511883Z	info	Successfully created new channel	{"chain_name": "kujira", "chain_id": "kujira", "channel_id": "channel-0", "connection_id": "connection-0", "port_id": "transfer"}
2023-10-12T10:06:13.948071Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 137128, "fees": "3088uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 18, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenConfirm"], "tx_hash": "86832DCCCCF56D0DFD7A6C28ACF93D415B8F0FAEFF2C5F9B262301C6876CE010"}
2023-10-12T10:06:14.512264Z	info	Successfully created new channel	{"chain_name": "terra", "chain_id": "terra", "channel_id": "channel-0", "connection_id": "connection-0", "port_id": "transfer"}
2023-10-12T10:06:14.512309Z	info	Channel handshake termination candidate	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0", "termination_port_id": "transfer", "observed_port_id": "transfer", "termination_counterparty_port_id": "transfer", "observed_counterparty_port_id": "transfer"}
2023-10-12T10:06:14.512316Z	info	Found termination condition for channel handshake	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0"}
==============> Starting relayers...<==============
```

### Upload the optimized cw binary and create a contract instance using following CLI commands.

For uploading,

```
kujirad tx wasm store {cw_root_dir/artifacts/kujira_ibc.wasm}   --from validator --gas auto --gas-adjustment 1.3 -y --output json --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

For instantiating,

```
kujirad tx wasm instantiate 1 '{}' --from validator --label "ibc" --gas auto --gas-adjustment 1.3 --no-admin -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

Now, we are all set for querying IBC through binding contract.

### CLI examples for IBC verification through contract

For querying **verify_membership**,

```
kujirad query wasm contract-state smart kujira14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sl4e867 '{"verify_membership":{"connection":"connection-0","revision_number":0,"revision_height":16,"path_prefix":"bank","path_key":"{path_key_bytes_base64}","proof":"{proof_bytes_base64}","value":"{value_bytes_base64}"}}'
```

For querying **verify_non_membership**,

```
kujirad query wasm contract-state smart kujira14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sl4e867 '{"verify_non_membership":{"connection":"connection-0","revision_number":0,"revision_height":16,"path_prefix":"bank","path_key":"{path_key_bytes_base64}","proof":"{proof_bytes_base64}"}}'
```

For both queries, they should return success if valid, or panic **error** if invalid.

```
data: success
```

### How to get path

Path is the combination of path prefix and path key.
Path prefix is usually module store key for the value you are going to verify and it is usually represented as module name.
Path key is the combination of all store sub prefixes and the right format for its representation is `{prefix1}/{prefix2}/{prefix3} `

Here is an example for getting path for user balance verification from other chain.
Lets say you are going to verify {uatom} balance at {cosmosxxx} address.
In KVStore of cosmos chain, uatom balance for the address is stored at following store key.

`{bytes(0x02)}{bytes("cosmosxxx")}{bytes("uatom")}`

In this case, path prefix is `bank`, and path key is `B/cosmosxxx/uatom`. (**B** is string representation of **0x02**)

Note1: Tendermint proof query should use `height-1`.
Note2: Revision height should use the one registered on chain.

- Example query to get raw bank storage proof.
  http://localhost:26657/abci_query?path=%22/store/bank/key%22&data=0x02144fea76427b8345861e80a3540a8a9d936fd39391756b756a69&prove=true&height=15
  `0x{02}{14}{4fea76427b8345861e80a3540a8a9d936fd39391}{756b756a69}`

- CLI command to get IBC storage proof

  ```sh
  $COUNTER_BINARY query ibc channel end transfer channel-0 --prove --height=16

  channel:
    connection_hops:
    - connection-0
    counterparty:
      channel_id: channel-0
      port_id: transfer
    ordering: ORDER_UNORDERED
    state: STATE_TRYOPEN
    version: ics20-1
  proof: CtQCCtECCi1jaGFubmVsRW5kcy9wb3J0cy90cmFuc2Zlci9jaGFubmVscy9jaGFubmVsLTASMggCEAEaFQoIdHJhbnNmZXISCWNoYW5uZWwtMCIMY29ubmVjdGlvbi0wKgdpY3MyMC0xGgsIARgBIAEqAwACHiIrCAESBAIEHiAaISAbtmnkQL7805qlBZC6nNLhjeF5vtUEa1tkbU2f6RhLnCIrCAESBAQGHiAaISAyOsT7i2laW6GLG+uWTGdqKL2qYo4pG6P3RUvc+h0PxSIrCAESBAYKHiAaISCuupVUqHKuPDgaZKxag/9NoHxbHbSQG9dOEo2XjI/fiyIrCAESBAgYHiAaISBnoLbwuv7Uw3oaiA8r/GnOlhxHhEjq/bfQ7Dw/jHbX4CIrCAESBAo0HiAaISCo3dvyBw2X/f0+V/lOHQH7lILSYpG9LFpCeNfpEfsRgAr8AQr5AQoDaWJjEiBKFJNuq3TholXRfu82UKXlueaM6zI8ujiu7lVcju3cExoJCAEYASABKgEAIiUIARIhAU4dXFY7DbD/3LpvyX3Ax7E7W1ycNBNX8IgnVlBH1sa5IiUIARIhAc5YxnFLhoktnqONBV6Gua4gAL/Wsk+kBp5NQz3yVM+2IicIARIBARogjiIcwtwqYUKG7VtRxjMVs28CrPJ6JhqjKEWj9BKiFo8iJQgBEiEB9yJO9LGvYehw7/7MpS0NvtAEZamBxp0cX0nTgaFnnzEiJwgBEgEBGiCLPkvCGKHz3QlwzfXHtksmh3kMTIWrrcqbUJS9sPgp3w==
  proof_height:
    revision_height: "16"
    revision_number: "0"
  ```

  KeyPath for IBC storage - to be converted to base64

  ```
  channelEnds/ports/transfer/channels/channel-0
  ```

  To get byte version of IBC storage

  ```
  http://localhost:26658/abci_query?path=%22/store/ibc/key%22&data=%22channelEnds/ports/transfer/channels/channel-0%22&prove=true&height=16

  {"jsonrpc":"2.0","id":-1,"result":{"response":{"code":0,"log":"","info":"","index":"0","key":"Y2hhbm5lbEVuZHMvcG9ydHMvdHJhbnNmZXIvY2hhbm5lbHMvY2hhbm5lbC0w","value":"CAIQARoVCgh0cmFuc2ZlchIJY2hhbm5lbC0wIgxjb25uZWN0aW9uLTAqB2ljczIwLTE=","proofOps":{"ops":[{"type":"ics23:iavl","key":"Y2hhbm5lbEVuZHMvcG9ydHMvdHJhbnNmZXIvY2hhbm5lbHMvY2hhbm5lbC0w","data":"CtECCi1jaGFubmVsRW5kcy9wb3J0cy90cmFuc2Zlci9jaGFubmVscy9jaGFubmVsLTASMggCEAEaFQoIdHJhbnNmZXISCWNoYW5uZWwtMCIMY29ubmVjdGlvbi0wKgdpY3MyMC0xGgsIARgBIAEqAwACHiIrCAESBAIEHiAaISAbtmnkQL7805qlBZC6nNLhjeF5vtUEa1tkbU2f6RhLnCIrCAESBAQGHiAaISAyOsT7i2laW6GLG+uWTGdqKL2qYo4pG6P3RUvc+h0PxSIrCAESBAYKHiAaISCuupVUqHKuPDgaZKxag/9NoHxbHbSQG9dOEo2XjI/fiyIrCAESBAgYHiAaISBnoLbwuv7Uw3oaiA8r/GnOlhxHhEjq/bfQ7Dw/jHbX4CIrCAESBAo0HiAaISCo3dvyBw2X/f0+V/lOHQH7lILSYpG9LFpCeNfpEfsRgA=="},{"type":"ics23:simple","key":"aWJj","data":"CvkBCgNpYmMSIEoUk26rdOGiVdF+7zZQpeW55ozrMjy6OK7uVVyO7dwTGgkIARgBIAEqAQAiJQgBEiEBTh1cVjsNsP/cum/JfcDHsTtbXJw0E1fwiCdWUEfWxrkiJQgBEiEBzljGcUuGiS2eo40FXoa5riAAv9ayT6QGnk1DPfJUz7YiJwgBEgEBGiCOIhzC3CphQobtW1HGMxWzbwKs8nomGqMoRaP0EqIWjyIlCAESIQGD2gQ/uK4gvcIeOSrbwj/buLB0HcUUOTICeMiQeGWKtCInCAESAQEaIP/LwmZkXCoIRjBj5nWa7V3eqeE6UjeRp+BVteSf6D6h"}]},"height":"16","codespace":""}}}
  ```

- Typescript to generate storage proof

  - package.json

  ```json
  {
    "name": "proof-test",
    "peerDependencies": {
      "typescript": "^5.0.0"
    },
    "dependencies": {
      "@cosmjs/encoding": "^0.31.1",
      "@cosmjs/tendermint-rpc": "^0.31.1",
      "@cosmjs/utils": "^0.31.1",
      "cosmjs-types": "^0.8.0",
      "kujira.js": "^0.9.33",
      "ts-node": "^10.9.2"
    }
  }
  ```

  - index.ts

  ```ts
  import { HttpClient, Tendermint37Client } from "@cosmjs/tendermint-rpc";
  import { kujiraQueryClient } from "kujira.js";
  import { fromBech32 } from "@cosmjs/encoding";
  import { CommitmentProof } from "cosmjs-types/cosmos/ics23/v1/proofs";
  import { MerkleProof } from "cosmjs-types/ibc/core/commitment/v1/commitment";
  (async function () {
    const addr = "terra1fl48vsnmsdzcv85q5d2q4z5ajdha8yu3nln0mh";
    const denom = "uluna";
    const rpc = "http://127.0.0.1:26658";
    const desiredHeight = 16;

    const rpcClient = new HttpClient(rpc);
    const tmClient = await Tendermint37Client.create(rpcClient);
    const querier = kujiraQueryClient({ client: tmClient });
    // query key is 0x2 | byte(address length) | []byte(address) | []byte(balance.Denom)
    const addrAsBytes = fromBech32(addr).data;
    const key = Uint8Array.from([
      0x2,
      addrAsBytes.length,
      ...addrAsBytes,
      ...Buffer.from(denom),
    ]);
    // console.log(key);
    const proof = await querier.queryRawProof("bank", key, desiredHeight - 1);

    const merkleProof = MerkleProof.fromPartial({
      proofs: proof.proof.ops.map((x) => CommitmentProof.decode(x.data)),
    });
    const proofBytes = Buffer.from(
      MerkleProof.encode(merkleProof).finish()
    ).toString("base64");

    const verifyMembership = {
      connection: "connection-0",
      revision_number: 1,
      revision_height: desiredHeight,
      proof: proofBytes,
      value: Buffer.from(proof.value).toString("base64"),
      path_prefix: "bank",
      path_key: Buffer.from(key).toString("base64"),
    };
    console.log(JSON.stringify(verifyMembership, null, 4));
  })().then(() => process.exit(0));
  ```

  - command & response

  ```sh
   npx ts-node index.ts
  ```

  ```json
  {
    "connection": "connection-0",
    "revision_number": 1,
    "revision_height": 16,
    "proof": "CuwBCukBChsCFE/qdkJ7g0WGHoCjVAqKnZNv05ORdWx1bmESCzEwMDAwMDAwMDAwGgsIARgBIAEqAwACAiIrCAESBAIEHiAaISC7ojWxH+9zBTVmRwT5Z2Q52n0FJtwl6zEqR43mbWxFeSIpCAESJQQGHiBV/kDCAkjL5M2VdNYMIOd3XrbbvO/iNsBz8mbw868tLyAiKwgBEgQGDB4gGiEgauSyPigyu0n/Ba3EQMT7lqWYrZo0JMIiAaQfAq4ux8AiKwgBEgQIFh4gGiEg7laYbBSoZxavTwhcXg8jj8dJOYmM32kZCBpq4LfoX8oK/wEK/AEKBGJhbmsSIJKe8fCaGp93+s38SzX232OgBW8Skq/vGb/YsIhlBKkcGgkIARgBIAEqAQAiJQgBEiEB9bjaf2bRNONCQldUmbHxJcB68hs2aFwx+aiZnHGiHa8iJQgBEiEBOZ73nZEyZFSTyQUorkNFXKY8RJ/GbRXGHEarwBi1dWMiJwgBEgEBGiBeUTktOrLSO0j3rAbWNh5VNL9QChI7En4HSHJit8ppWSInCAESAQEaIJhOsQmSZmh/fe41aTep1F3dQiAx/fv0Q0ZnaP8CqgSoIicIARIBARogiz5Lwhih890JcM31x7ZLJod5DEyFq63Km1CUvbD4Kd8=",
    "value": "MTAwMDAwMDAwMDA=",
    "path_prefix": "bank",
    "path_key": "AhRP6nZCe4NFhh6Ao1QKip2Tb9OTkXVsdW5h"
  }
  ```
