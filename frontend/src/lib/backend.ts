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
    return null
}

export async function getWorldRecords(event?: string|null) {
    try {
        let path = `${BACKEND}/json/all_puzzles_leaderboard`
        if (event !== undefined) { path = (`${BACKEND}/json/all_puzzles_leaderboard?event=${event}`) }

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

export async function getPuzzleSolves(id: number, event?: string|null) {
    try {
        let path = `${BACKEND}/json/puzzle?id=${id}`
        if (event !== undefined) {path = `${BACKEND}/json/puzzle?id=${id}&event=${event}`}
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
