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

export type Variant = {
    id: number,
    name: string,
    prefix: string,
    suffix: string,
    abbr: string
}

export type Speed = {
    average: boolean,
    blind: boolean,
    filters: boolean,
    macros: boolean,
    one_handed: boolean,
    variant?: Variant,
    material: boolean
}

export type Fmc = {
    computer_assissted: boolean
}

export type Category = {
    cat: Speed | Fmc
}

export type Event = {
    puzzle: Puzzle,
    category: Category
}

export type SolveFlags = {
    average: boolean,
    blind: boolean,
    filters: boolean,
    macros: boolean,
    one_handed: boolean,
    computer_assissted: boolean
}

export type Program = {
    id: number,
    name: string,
    abbr: string,
    material: boolean
}

export type PublicUser = {
    id: number,
    name?: string
}

export type FullSolve = {
    id: number,
    // Metadata
    solve_date: string,
    upload_date: string,
    solver_notes?: string,
    moderator_notes?: string,
    auto_verify_output?: JSON,
    // Event
    puzzle: Puzzle,
    variant?: Variant,
    flags: SolveFlags,
    program: Program
    // Score
    move_count?: number,
    speed_cs?: number,
    memo_cs?: number,
    // Verification
    fmc_verified?: boolean,
    fmc_verified_by?: number,
    speed_verified?: boolean,
    speed_verified_by?: boolean,
    // Evidence
    log_file_name?: string,
    scramble_seed?: string,
    video_url?: string,
    // Solver
    solver: PublicUser
}

export type RankedFullSolve = {
    rank: number,
    solve: FullSolve
}

export type Record = [Event, FullSolve]





// ------------------------------------------
// functions to call from the frontend
// ------------------------------------------




export async function getPuzzles() {
    try {
        const res = await fetch(`${BACKEND}/json/puzzles`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getWorldRecords() {
    try {
        const res = await fetch(`${BACKEND}/json/all_puzzles_leaderboard`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getSolve(id: number) {
    try {
        const res = await fetch(`${BACKEND}/json/solve?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getPuzzleSolves(id: number) {
    try {
        const res = await fetch(`${BACKEND}/json/puzzle?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}
