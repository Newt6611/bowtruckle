<p align="center">
  <img src="logo.png" alt="Bowtruckle logo" width="320">
</p>

# Bowtruckle

Bowtruckle turns Cardano transaction CBOR hex into readable Markdown.

It is meant for workflows where you want the decoded transaction in a plain,
editable document instead of a TUI or a JSON blob. Print it in the terminal,
pipe it into Neovim, or save it as a Markdown file.

## Features

- Parses Cardano transaction CBOR with `cardano-serialization-lib`.
- Accepts raw CBOR hex directly or reads it from a file.
- Prints Markdown to stdout by default.
- Can write Markdown to a file with `-o` / `--output`.
- Expands the full transaction structure, including body, inputs, outputs,
  witness set, auxiliary data, metadata, fees, signers, datum, and scripts.
- Decodes native asset names from asset-name hex when they are valid UTF-8,
  while preserving the original hex.

## Usage

```bash
bowtruckle <RAW_CBOR_HEX|CBOR_FILE> [-o OUTPUT]
```

### Print Markdown To The Terminal

```bash
bowtruckle 84a700...
```

### Read CBOR Hex From A File

```bash
bowtruckle tx.cbor
```

### Save Markdown To A File

```bash
bowtruckle tx.cbor -o tx.md
```

or:

```bash
bowtruckle 84a700... --output tx.md
```

### Pipe Into Neovim

```bash
bowtruckle 84a700... | nvim
```

When developing locally with Cargo:

```bash
cargo run -- 84a700...
cargo run -- tx.cbor -o tx.md
```

`cargo run . 84a700...` is also tolerated for convenience.

## Output Shape

Bowtruckle renders a Markdown document with stable sections:

```markdown
# Cardano Transaction

**Transaction Hash:** `...`

## Transaction

### Body

#### Inputs
- **#0**
  - **Index:** `5`
  - **Transaction Id:** `...`

#### Outputs
- **#0**
  - **Address:** `addr...`
  - **Amount**
    - **Coin:** `4000000`
    - **Multiasset**
      - policy `f13ac4...`
        - asset `535452494b45` (`STRIKE`) qty `500000000`

#### Other Body Fields
- **Fee:** `...`
- **Required Signers**

### Witness Set

### Auxiliary Data
```

For native assets, the format is:

```markdown
- asset `<asset-name-hex>` (`<decoded-name-or-non-utf8>`) qty `<quantity>`
```

This keeps the raw on-chain bytes visible while still making common token names
easy to read.

## Build

```bash
cargo build --release
```

The binary will be at:

```bash
target/release/bowtruckle
```

## Test

```bash
cargo test
```
