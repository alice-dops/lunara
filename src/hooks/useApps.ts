import { useCallback, useEffect, useState } from "react"
import { R_AppView } from "../types/types"
import { listApps } from "../services/apps"


export function useApps() {
	const [apps, setApps] = useState<R_AppView[]>([])
	const [loading, setLoading] = useState(false)
	const [error, setError] = useState<string | null>(null)

	const reload = useCallback(async () => {
		try {
			setLoading(true)
			setError(null)

			const data = await listApps()
			setApps(data)
		} catch (err) {
			console.error("Failed to load apps:", err)
			setError("Failed to load apps")
		} finally {
			setLoading(false)
		}
	}, [])

	useEffect(() => {
		reload()
	}, [reload])

	return {
		apps,
		loading,
		error,
		reload
	}
}

