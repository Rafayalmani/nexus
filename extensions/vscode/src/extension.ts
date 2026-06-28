import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
	console.log('Nexus VS Code extension activated');

	let disposable = vscode.commands.registerCommand(
		'nexus.syncWorkspace',
		() => {
			vscode.window.showInformationMessage('Nexus: Syncing workspace...');
		}
	);

	context.subscriptions.push(disposable);
}

export function deactivate() {}
