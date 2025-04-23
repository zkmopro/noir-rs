# Noir-rs

**Noir‑rs** is a Rust crate that turns Noir circuits into Rust and (optionally) links the Barretenberg prover for Ultra‑Honk proofs. It works on macOS, Linux, iOS (aarch64‑apple‑ios), and Android (aarch64‑linux‑android).

## Installation

```toml
# Cargo.toml
[dependencies]
noir = { git = "https://github.com/zkmopro/noir-rs", features = ["barretenberg"] }

# For Android add the `android-compat` feature:
noir = { git = "https://github.com/zkmopro/noir-rs", features = ["barretenberg", "android-compat"] }
```

## Platform Support

- macOS
- Linux (x86‑64)
- iOS (aarch64‑apple‑ios)
- Android (aarch64‑linux‑android)

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

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
