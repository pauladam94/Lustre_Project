install-lsp:
    cargo install --path lsp

build-vscode-extension:
    cargo build --release
    cp target/release/lustrels vscode_extension/server/lustrels
    cd vscode_extension && vsce package --out lustre-vscode.vsix
    codium --install-extension vscode_extension/lustre-vscode.vsix 
