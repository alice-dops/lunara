import { IconName } from "lucide-react/dynamic"

export interface R_SnippetView {
	snippet: R_Snippet
	query?: string
}

export interface SnippetViewDisplay {
	snippet: R_Snippet
	query?: string
	display: DisplaySnippet
	parseError?: string
}


export interface DisplaySnippet {
	name: string
	icon: IconName
	disable?: boolean
}

export interface R_Snippet {
	id: string
	name: string
	confirmation?: boolean
	icon: IconName
	executor: R_Snippet_Executor[]
}

export interface R_Snippet_Executor {
	actionButton: string
}

export interface R_AppView {
	app: AppEntry
}

export interface AppEntry {
	id: string
	name: string
	cmd: string[]
	wm_class?: string | null
	icon?: string | null
	isRunning?: boolean
}

export interface R_SystemStatus {
	volume_percent: number;
	muted: boolean;
	battery_percent: number | null;
	charging: boolean | null;
	wifi_up: boolean;
	ssid: string | null;
};
