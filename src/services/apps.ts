import { invoke } from "@tauri-apps/api/core"
import { R_AppView, R_Snippet, R_SystemStatus } from "../types/types"
import { ActionBtn } from "../contexts/lunarInputContext";

export async function listApps(): Promise<R_AppView[]> {
	return await invoke("list_apps");
}

export async function listSnippets(): Promise<R_Snippet[]> {
	return await invoke("list_snippets");
}

export async function runApp(id: string): Promise<void> {
	return await invoke("run_app", { id: id });
}

export async function runSinppet(id: string, button: ActionBtn): Promise<void> {
	return await invoke("run_snippet", { id, button })
}

export async function getSystemStatus(): Promise<R_SystemStatus> {
	return await invoke("get_system_status");
}

