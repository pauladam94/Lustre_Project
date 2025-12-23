import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from "vscode-languageclient/node";

let client: LanguageClient;

function getExePath(platform: string): string {
    if (platform === "win32") {
        return "lustrels.exe";
    } else if (platform === "darwin") {
        return "lustrels.darwin";
    } else {
        return "lustrels";
    }
}
  
export function activate(context: vscode.ExtensionContext) {
  
  const serverExe = getExePath(process.platform);

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

