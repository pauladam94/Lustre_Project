import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  // Path to the LSP binary
  const serverExe = process.platform === "win32"
    ? "lustrels.exe"
    : "lustrels";

  // Full path to where we keep the LSP binary inside the extension
  const serverPath = context.asAbsolutePath(
    path.join("server", serverExe)
  );

  const serverOptions: ServerOptions = {
    run: { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio }
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "lustre" }],
    outputChannel: vscode.window.createOutputChannel("Lustre Language Server"),
  };

  client = new LanguageClient(
    "lustreLsp",
    "Lustre Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client ? client.stop() : undefined;
}
