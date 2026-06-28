// many functions and types for getting data from the backend
const BACKEND = "http://localhost:3000"

export type Puzzle = {
    id: number,
    name: string,
    primary_filters: boolean,
    primary_macros: boolean,
    hsc_id: string | null,
    autoverifiable: boolean
  }

export async function getPuzzles() {
    try {
        const res = await fetch(`${BACKEND}/json/puzzles`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}
