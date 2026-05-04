import * as vscode from "vscode";

export class EngageCodeActionProvider implements vscode.CodeActionProvider {
    public static readonly providedKinds = [vscode.CodeActionKind.QuickFix];

    provideCodeActions(
        _document: vscode.TextDocument,
        _range: vscode.Range,
        context: vscode.CodeActionContext,
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        for (const diag of context.diagnostics) {
            if (diag.source !== "engage") {
                continue;
            }

            if (diag.code === "missing-features") {
                const fix = new vscode.CodeAction("engage: Apply missing features", vscode.CodeActionKind.QuickFix);
                fix.command = { command: "engage.featuresApply", title: "Apply missing features" };
                fix.diagnostics = [diag];
                fix.isPreferred = true;
                actions.push(fix);
            }
        }

        return actions;
    }
}

export function registerCodeActions(context: vscode.ExtensionContext): void {
    context.subscriptions.push(
        vscode.languages.registerCodeActionsProvider(
            { scheme: "file", pattern: "**/Cargo.toml" },
            new EngageCodeActionProvider(),
            { providedCodeActionKinds: EngageCodeActionProvider.providedKinds },
        ),
    );
}
