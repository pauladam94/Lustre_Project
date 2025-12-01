install-lsp:
    cargo install --path lsp

build-vscode-extension:
    cargo build --release --target x86_64-unknown-linux-gnu --bin lustrels
    cp target/release/lustrels vscode_extension/server/lustrels.linux

    # cargo build --release --target wasm32-unknown-unknown --bin lustrels
    # cp target/release/lustrels vscode_extension/server/lustrels.wasm

    # cargo build --release --target x86_64-unknown-linux-gnu --bin lustrels
    # cp target/release/lustrels vscode_extension/server/lustrels.linux

    cd vscode_extension && vsce package --out lustre-vscode.vsix
    codium --install-extension vscode_extension/lustre-vscode.vsix 

