import type { Temporal } from "@js-temporal/polyfill"
import { category_query_to_url_params } from "./utils"

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

export type CombinedVariant = {
    name: string,
    variant_abbr?: string
    program?: string
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
    Speed?: Speed,
    Fmc?: Fmc
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

export type SelfInfoResponse = {
    id: number,
    name?: string,
    email?: string,
    discord_id?: number,
    discord_username?: string,
    discord_nickname?: string,
    discord_avatar_url?: string,
    moderator: boolean,
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

export type MainPageCategory = {
    Speed: {
        puzzle: number,
        variant?: number
        material: boolean
    },
    Fmc: {
        puzzle: number
    }
}

export type PB = [MainPageCategory, RankedFullSolve]

export type CategoryQuery = {
    Speed: {
        average: boolean,
        blind: boolean,
        filters?: boolean,
        macros?: boolean,
        one_handed: boolean,
        variant: string,
        program: string
    },
    Fmc: {
        enabled: boolean,
        computer_assisted: boolean
    }
}




// ------------------------------------------
// get functions
// ------------------------------------------


export async function getVariants() {
    try {
        const res = await fetch(`${BACKEND}/json/variants`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function getCombinedVariants(id: number) {
    try {
        const res = await fetch(`${BACKEND}/json/combinedvariants?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}


export async function getPrograms() {
    try {
        const res = await fetch(`${BACKEND}/json/programs`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}


export async function getPuzzles() {
    try {
        const res = await fetch(`${BACKEND}/json/puzzles`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

// Needs to take category query instead of just the event string
// export async function getWorldRecords(event?: string|null) {
//     try {
//         let path = `${BACKEND}/json/all_puzzles_leaderboard`
//         if (event !== undefined) { path = (`${BACKEND}/json/all_puzzles_leaderboard?event=${event}`) }

//         const res = await fetch(path)

//         if (! res.ok) return null
//         return res.json()
//     } catch(err) {
//         console.log("error fetching data", err)
//     }
// }
export async function getWorldRecords(query: CategoryQuery | undefined) {
    category_query_to_url_params(query)
    try {
        let path = `${BACKEND}/json/all_puzzles_leaderboard?${category_query_to_url_params(query)}`
        const res = await fetch(path)

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

export async function getPuzzleSolves(id: number, query: CategoryQuery | undefined) {
    try {
        let path = `${BACKEND}/json/puzzle?id=${id}&${category_query_to_url_params(query)}`
        const res = await fetch(path)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getUserPbs(id: number) {
    try {
        const res = await fetch(`${BACKEND}/json/user/pbs?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getUserSubmissions(id?: number|null) {
    try {
        const res = await fetch(`${BACKEND}/json/user/submissions?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}



// ------------------------------------------
// post functions
// ------------------------------------------

export type SolveData = {
    solve_id?: number,

    // Event
    puzzle_id: number
    variant_id?: number,
    program_id: number,

    // Metadata
    solver_id?: number,
    solve_date: Temporal.PlainDate
    solver_notes?: string,
    moderator_notes?: string,

    //Speedsolve
    solve_h?: number,
    solve_m?: number,
    solve_s?: number,
    solve_cs?: number,
    uses_filters: boolean,
    uses_macros: boolean,
    average: boolean,
    one_handed: boolean,
    blind: boolean,
    memo_h?: number,
    memo_m?: number,
    memo_s?: number,
    memo_cs?: number,
    video_url?: string,

    //Fewest moves
    move_count?: number,
    computer_assisted: boolean,
    replace_log_file?: boolean,
    log_file?: File

    audit_log_comment?: string,
}

export type UpdateSolveResponse = {
    solve_id: number
}



export async function submitSolve(data: SolveData): Promise<UpdateSolveResponse|null> {

    const formData = new FormData
    const appendIfDefined = (key: string, value: string | number | boolean | File | undefined) => {
        if (value === undefined) {
            return
        }

        if (value instanceof File) {
            formData.append(key, value)
            return
        }

        formData.append(key, value.toString())
    }

    appendIfDefined("solve_id", data.solve_id)
    appendIfDefined("puzzle_id", data.puzzle_id)
    appendIfDefined("variant_id", data.variant_id)
    appendIfDefined("program_id", data.program_id)
    appendIfDefined("solver_id", data.solver_id)
    appendIfDefined("solve_date", data.solve_date.toString())
    appendIfDefined("solver_notes", data.solver_notes)
    appendIfDefined("moderator_notes", data.moderator_notes)
    appendIfDefined("solve_h", data.solve_h)
    appendIfDefined("solve_m", data.solve_m)
    appendIfDefined("solve_s", data.solve_s)
    appendIfDefined("solve_cs", data.solve_cs)
    appendIfDefined("uses_filters", data.uses_filters)
    appendIfDefined("uses_macros", data.uses_macros)
    appendIfDefined("average", data.average)
    appendIfDefined("one_handed", data.one_handed)
    appendIfDefined("blind", data.blind)
    appendIfDefined("memo_h", data.memo_h)
    appendIfDefined("memo_m", data.memo_m)
    appendIfDefined("memo_s", data.memo_s)
    appendIfDefined("memo_cs", data.memo_cs)
    appendIfDefined("video_url", data.video_url)
    appendIfDefined("move_count", data.move_count)
    appendIfDefined("computer_assisted", data.computer_assisted)
    appendIfDefined("replace_log_file", data.replace_log_file)
    appendIfDefined("log_file", data.log_file)
    appendIfDefined("audit_log_comment", data.audit_log_comment)

    try {
        const res = await fetch(`${BACKEND}/submit-solve`, {
                method: 'POST',
                body: formData,
                credentials: 'include',
            })
        if (! res.ok) {
            throw new Error(`HTTP error! Status: ${res.status}`);
        }

        return res.json()
    }
        catch(err) {
        console.log("error fetching data", err)
    }
    return null

}



// ------------------------------------------
// Authentication functions
// ------------------------------------------

export type User = {
    id: number,
    email?: string,
    discord_id?: number,
    name?: string,
    moderator: boolean,
    moderator_notes: string,
    dummy: boolean
}

export type OtpResponse = {
    user?: User,
    device_code: string,
    auth_type: string
}

export type SubmitOtpRequest = {
    device_code: string,
    otp: string
}

export type SignInDiscordRequest = {
    username: string,
    turnstile_response?: string,
    redirect?: string
}

export async function requestOtpDiscord(data: SignInDiscordRequest): Promise<OtpResponse|null> {
    try {
        const formData = new FormData()
        formData.append("username", data.username)
        if (data.turnstile_response !== undefined) {
            formData.append("turnstile_response", data.turnstile_response)
        }
        if (data.redirect !== undefined) {
            formData.append("redirect", data.redirect)
        }

        const res = await fetch(`${BACKEND}/request-otp-discord`, {
            method: 'POST',
            body: formData,
            credentials: 'include',
        })
        if (! res.ok) {
            throw new Error(`HTTP error! Status: ${res.status}`);
        }

        const html = await res.text()
        const document = new DOMParser().parseFromString(html, "text/html")
        const deviceCode = document.querySelector('input[name="device_code"]')?.getAttribute("value")
        if (!deviceCode) {
            throw new Error("Missing device code in OTP response")
        }

        return {
            device_code: deviceCode,
            auth_type: "DiscordOtp",
        }
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function getCurrentUser(): Promise<SelfInfoResponse | null> {
    try {
        const res = await fetch(`${BACKEND}/self-info`, {
            credentials: 'include',
        })
        if (!res.ok) {
            return null
        }

        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }

    return null
}

export async function signOut(): Promise<boolean> {
    try {
        const res = await fetch(`${BACKEND}/sign-out`, {
            credentials: 'include',
        })
        return res.ok
    } catch(err) {
        console.log("error fetching data", err)
    }

    return false
}

export async function submitOtpRequest(data: SubmitOtpRequest): Promise<string|null> {
    try {
        const formData = new FormData()
        formData.append("device_code", data.device_code)
        formData.append("otp", data.otp)

        const res = await fetch(`${BACKEND}/submit-otp`, {
            method: 'POST',
            body: formData,
            credentials: 'include',
        })
        if (! res.ok) return null

        const redirectedUrl = new URL(res.url)
        return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
    } catch(err) {
        console.log("error fetching data", err)
    }

    return null
}
