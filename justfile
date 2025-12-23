install-lsp:
    cargo install --path lsp

build-to TARGET-TRIPLE FILE:
    cargo build --release --target {{TARGET-TRIPLE}} --bin lustrels
    cp target/{{TARGET-TRIPLE}}/release/{{FILE}} vscode_extension/server/lustrels


build-vscode-extension:
    rm -f vscode_extension/server/*

    just build-to x86_64-unknown-linux-gnu lustrels

    # just build-to wasm32-wasip1 lustrels.wasm

    cd vscode_extension && vsce package --out lustre-vscode.vsix
    codium --install-extension vscode_extension/lustre-vscode.vsix 

