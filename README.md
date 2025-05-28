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

-   macOS
-   Linux (x86‑64)
-   iOS
    -   aarch64‑apple‑ios
    -   aarch64-apple-ios-sim
    -   x86_64-apple-ios
-   Android
    -   aarch64‑linux‑android
    -   x86_64-linux-android

## Downloading SRS (Structured Reference String)

Noir requires a Structured Reference String (SRS) for its operations. You can download the necessary SRS files using the `srs_downloader` utility included in the `noir` crate.

### 1. SRS for a Specific Circuit

If you have a compiled circuit (e.g., `circuit.json` from `nargo compile`), you can download the SRS tailored for that circuit.

```sh
cargo run --bin srs_downloader --features srs-downloader -- -c path/to/your/circuit.json
```

This will download the SRS and save it to `./srs_cache/your_circuit_name.srs` by default (e.g., `./srs_cache/my_circuit.srs`).

### 2. Default SRS

If you don't have a specific circuit manifest or want a general-purpose SRS, you can download a default one (supports up to 2^18 constraints):

```sh
cargo run --bin srs_downloader --features srs-downloader
```

This will download the SRS and save it to `./srs_cache/default_18.srs` by default.

**Custom Output Path:**

You can specify a custom output path for the downloaded SRS file using the `-o` flag:

```sh
# For a specific circuit
cargo run --bin srs_downloader --features srs-downloader -- -c path/to/your/circuit.json -o /custom/path/to/srs_file.srs

# For a default SRS
cargo run --bin srs_downloader --features srs-downloader -- -o /custom/path/to/default.srs
```

The tool will automatically create parent directories if they don't exist.

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

const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";    // output of `nargo compile`

fn main() {
    /// Download SRS via `srs_downloader`:
    /// - Circuit-specific (`-c path/to/my_circuit.json`): `./srs_cache/my_circuit.srs`
    /// - Default (no `-c`): `./srs_cache/default_18.srs`
    ///

    // 1. Update srs_path to the location of your downloaded SRS file.
    // (Option 1)
    // let srs_path = "./srs_cache/my_circuit.srs";
    // setup_srs_from_bytecode(BYTECODE, Some(srs_path), false).unwrap();

    // (Option 2)
    // Alternatively, if you know the circuit size, you can use the following function
    // Assuming the circuit size is 40 here
    setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    setup_srs(40, None).unwrap();

    // 2. Witness: a = 5, b = 6, res = 30
    let witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();

    // 3. Prove
    let proof = prove_ultra_honk(BYTECODE, witness, false).unwrap();

    // 4. Verify
    let vk = get_honk_verification_key(BYTECODE, false).unwrap();
    let is_valid = verify_ultra_honk(proof, vk).unwrap();
    println!("✔ proof valid? {:?}", is_valid);
}
```

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
