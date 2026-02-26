import { useEffect, useState } from "react";
import { R_SystemStatus } from "../types/types";
import { listen } from "@tauri-apps/api/event";
import { getSystemStatus } from "../services/apps";

export function useSystemStatus() {
	const [status, setStatus] = useState<R_SystemStatus | null>(null);

	useEffect(() => {
		getSystemStatus().then(setStatus);

		let unlisten: null | (() => void) = null;

		(async () => {
			unlisten = await listen<R_SystemStatus>("lunara://system", (e) => {
				setStatus(e.payload);
			});
		})();

		return () => {
			unlisten?.();
		};
	}, []);

	return status;
}
