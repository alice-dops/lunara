import { useCallback, useEffect, useState } from "react"
import { R_Snippet } from "../types/types"
import { listSnippets } from "../services/apps"


export function useSnippets() {
	const [snippets, setSnippets] = useState<R_Snippet[]>([])
	const [loading, setLoading] = useState(false)
	const [error, setError] = useState<string | null>(null)

	const reload = useCallback(async () => {
		try {
			setLoading(true)
			setError(null)

			const data = await listSnippets()
			setSnippets(data)
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
		snippets: snippets,
		loading,
		error,
		reload
	}
}

