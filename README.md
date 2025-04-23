# Noir-rs

**Noir‑rs** is a Rust workspace that turns Noir circuits into Rust crates and (optionally) links the Barretenberg prover for blazing‑fast Ultra‑Honk proofs. With one command from the **mopro CLI**, it can generate Swift and Kotlin bindings so the **same circuit runs natively on iOS and Android** with zero glue code required.

## Installation

Add the crate to your project directly from GitHub:

```toml
# Cargo.toml
[dependencies]
noir = { git = "https://github.com/zkmopro/noir-rs", features = ["barretenberg"] }

# For Android builds also enable android-compat:

noir = { git = "https://github.com/zkmopro/noir-rs",
         features = ["barretenberg", "android-compat"] }
```

The workspace layout lets you keep Barretenberg optional while still using pure-Rust back-ends where they exist.

## Build the Bindings

We use the mopro CLI to generate Noir-rs bindings for mobile apps. You can build these bindings in `debug` or `release` mode using the interactive CLI. If you haven’t installed the CLI yet, please refer to the [zkmopro docs](https://zkmopro.org/docs/getting-started/#1-install-cli).

### iOS

Run

```sh
mopro build
```

and select `aarch64-apple-ios`

### Android

Activate `android-compat` feature in [Cargo.toml](./noir/Cargo.toml).

```diff
- noir = { git = "https://github.com/zkmopro/noir-rs", features = ["barretenberg"] }
+ noir = { git = "https://github.com/zkmopro/noir-rs", features = ["barretenberg", "android-compat"] }
```

Run

```sh
mopro build
```

and select `aarch64-linux-android`


## Quick Start

Below is a minimal `a * b = res` circuit proof. Compile your circuit with nargo compile, copy the bytecode string into BYTECODE, then:
```rs
use noir::{
    barretenberg::{
        prove::prove_ultra_honk,
        srs::{setup_srs_from_bytecode, setup_srs},
        utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

const BYTECODE: &str = "...";      // output of `nargo compile`

fn main() {
    // 1. Structured Reference String
    setup_srs_from_bytecode(BYTECODE, None, false).unwrap();

    // 2. Witness: a = 5, b = 6, res = 30
    let witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();

    // 3. Prove
    let proof = prove_ultra_honk(BYTECODE, witness).unwrap();

    // 4. Verify
    let vk = get_honk_verification_key(BYTECODE).unwrap();
    let isValid = verify_ultra_honk(proof, vk).unwrap();
    println!("✔ proof valid? {isValid}");
}
```

To create APIs for your mobile applications that leverage above Noir functionalities, you can use `mopro init` to initialize the adapter and follow the current guidance to integrate `noir-rs` into it.

Once it has been built, you can scaffold a mobile app that runs the example above, using the `mopro create` command as described in the [zkmopro docs](https://zkmopro.org/docs/getting-started/#4-create-templates).

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
