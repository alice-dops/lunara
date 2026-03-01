import { useEffect, useState } from "react"
import { R_SnippetView, SnippetViewDisplay } from "../types/types"
import { listSnippets } from "../services/apps"
import { listen } from "@tauri-apps/api/event"


function parseSnippet(snippet: R_SnippetView): SnippetViewDisplay {
	let r: SnippetViewDisplay = {
		snippet: snippet.snippet,
		query: snippet.query,
		display: {
			name: snippet.snippet.name,
			icon: snippet.snippet.icon,
			disable: false
		}
	}
	console.log(snippet)

	if (!snippet.query) return r

	try {
		let parsed = JSON.parse(snippet.query);
		r.display = {
			name: parsed.name ?? snippet.snippet.name,
			icon: parsed.icon ?? snippet.snippet.icon,
			disable: parsed.disable ?? false,
		}
	} catch (e: any) {
		console.error(e);
		r.parseError = e.message;
	} finally {
		console.log(r)
		return r;
	}
}

export function useSnippets() {
	const [snippets, setSnippets] = useState<SnippetViewDisplay[]>([])

	useEffect(() => {
		listSnippets().then((s) => setSnippets(s.map(parseSnippet).filter((s) => !s.display.disable)));

		let unlisten: null | (() => void) = null;

		(async () => {
			unlisten = await listen<R_SnippetView[]>("lunara://snippets", (e) => {
				setSnippets(e.payload.map(parseSnippet).filter((s) => !s.display.disable));
			});
		})();

		return () => {
			unlisten?.();
		};
	}, [])

	return {
		snippets: snippets,
	}
}

