import * as vscode from "vscode";

const REPO = "DivineDragonFanClub/engage-il2cpp";
const TAG_PREFIX = "vscode-v";
const CACHE_KEY = "engage.updater.lastCheck";
const ETAG_KEY = "engage.updater.lastEtag";
const CHECK_INTERVAL_MS = 24 * 60 * 60 * 1000;

interface GitHubRelease {
    tag_name: string;
    assets: { name: string; browser_download_url: string }[];
}

export function activateUpdater(context: vscode.ExtensionContext, output: vscode.OutputChannel): void {
    const mode = vscode.workspace.getConfiguration("engage").get<string>("autoUpdateCheck", "prompt");

    if (mode === "off") {
        return;
    }

    setTimeout(() => {
        checkForUpdates(context, output, mode === "auto").catch((err) => {
            output.appendLine(`update check failed: ${err}`);
        });
    }, 5000);

    context.subscriptions.push(
        vscode.commands.registerCommand("engage.checkForUpdates", () => {
            checkForUpdates(context, output, false).catch((err) => {
                vscode.window.showErrorMessage(`engage: update check failed. ${err}`);
            });
        }),
    );
}

async function checkForUpdates(context: vscode.ExtensionContext, output: vscode.OutputChannel, autoApply: boolean): Promise<void> {
    const lastCheck = context.globalState.get<number>(CACHE_KEY, 0);
    const now = Date.now();

    if (now - lastCheck < CHECK_INTERVAL_MS) {
        return;
    }

    const release = await fetchLatestRelease(context.globalState.get<string>(ETAG_KEY));

    if (release === "not-modified") {
        await context.globalState.update(CACHE_KEY, now);
        return;
    }

    if (!release) {
        return;
    }

    await context.globalState.update(CACHE_KEY, now);

    const currentVersion = context.extension.packageJSON.version as string;
    const latestVersion = release.tag_name.replace(TAG_PREFIX, "").replace(/^v/, "");

    if (!isNewer(latestVersion, currentVersion)) {
        return;
    }

    const vsix = release.assets.find((a) => a.name.endsWith(".vsix") && matchesPlatform(a.name));

    if (!vsix) {
        output.appendLine(`update ${latestVersion} available, but no VSIX matches this platform`);
        return;
    }

    if (autoApply) {
        await applyUpdate(vsix.browser_download_url, output);
        return;
    }

    const choice = await vscode.window.showInformationMessage(
        `engage ${latestVersion} is available. (current: ${currentVersion})`,
        "Update",
        "Skip",
    );

    if (choice === "Update") {
        await applyUpdate(vsix.browser_download_url, output);
    }
}

async function fetchLatestRelease(etag: string | undefined): Promise<GitHubRelease | "not-modified" | undefined> {
    const url = `https://api.github.com/repos/${REPO}/releases/latest`;
    const headers: Record<string, string> = {
        Accept: "application/vnd.github+json",
        "User-Agent": "engage-vscode",
    };

    if (etag) {
        headers["If-None-Match"] = etag;
    }

    const response = await fetch(url, { headers });

    if (response.status === 304) {
        return "not-modified";
    }

    if (!response.ok) {
        return undefined;
    }

    return (await response.json()) as GitHubRelease;
}

async function applyUpdate(vsixUrl: string, output: vscode.OutputChannel): Promise<void> {
    output.appendLine(`installing update from ${vsixUrl}`);

    try {
        await vscode.commands.executeCommand("workbench.extensions.installExtension", vscode.Uri.parse(vsixUrl));
    } catch (err) {
        const message = err instanceof Error ? err.message : String(err);

        vscode.window.showErrorMessage(`engage: install failed. ${message}`);
        return;
    }

    const reload = await vscode.window.showInformationMessage("engage update installed. Reload to activate.", "Reload now");

    if (reload === "Reload now") {
        vscode.commands.executeCommand("workbench.action.reloadWindow");
    }
}

function matchesPlatform(assetName: string): boolean {
    const platform = process.platform;
    const arch = process.arch;

    if (platform === "darwin" && arch === "arm64") {
        return assetName.includes("darwin-arm64");
    }

    if (platform === "darwin" && arch === "x64") {
        return assetName.includes("darwin-x64");
    }

    if (platform === "linux" && arch === "x64") {
        return assetName.includes("linux-x64");
    }

    if (platform === "win32" && arch === "x64") {
        return assetName.includes("win32-x64");
    }

    return false;
}

function isNewer(a: string, b: string): boolean {
    const split = (s: string) => s.split(".").map((p) => parseInt(p, 10) || 0);
    const aP = split(a);
    const bP = split(b);

    for (let i = 0; i < Math.max(aP.length, bP.length); i++) {
        const ai = aP[i] ?? 0;
        const bi = bP[i] ?? 0;

        if (ai > bi) {
            return true;
        }

        if (ai < bi) {
            return false;
        }
    }

    return false;
}
