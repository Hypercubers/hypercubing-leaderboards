import { category_query_to_url_params } from "./utils"

// many functions and types for getting data from the backend
const BACKEND = "http://localhost:3000"














// ------------------------------------------
// get functions
// ------------------------------------------





export async function getWorldRecords(query: CategoryQuery | undefined) {

    const path = `${BACKEND}/json/all-puzzles-leaderboard?${category_query_to_url_params(query)}`
    const res = await fetch(path)

    if (! res.ok) {
        throw new Error(`Failed to fetch world records: ${res.status}`)
    }

    return res.json()
   
}

export async function getDistinctRecords() {
    const path = `${BACKEND}/json/distinct_puzzles_leaderboards`
    const res = await fetch(path)

    if (! res.ok) {
        throw new Error(`Failed to fetch distinct records: ${res.status}`)
    }
    return res.json()
}

export async function getRecordHistory(id: number, query: CategoryQuery | undefined) {
    try {
        const path = `${BACKEND}/json/recordhistory?id=${id}&${category_query_to_url_params(query)}`
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

export async function getSolveLogs(id: number) {
    try {
        const res = await fetch(`${BACKEND}/json/solve-logs?id=${id}`)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getPuzzleSolves(id: number, query: CategoryQuery | undefined) {
    try {
        const path = `${BACKEND}/json/puzzle?id=${id}&${category_query_to_url_params(query)}`
        const res = await fetch(path)
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
}

export async function getUserPbs(id: number, query: CategoryQuery | undefined): Promise<PB[]|null> {
    const res = await fetch(`${BACKEND}/json/user/pbs?id=${id}&${category_query_to_url_params(query)}`)
    if (! res.ok) {
        throw new Error(`Failed to fetch user PBs: ${res.status}`)
    }
    return res.json()
    
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

export async function getUser(id: number): Promise<PublicUser|null> {
    const res = await fetch(`${BACKEND}/json/user?id=${id}`)
    if (! res.ok) {
        throw new Error(`Failed to fetch user: ${res.status}`)
    }
    return res.json()
    
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
    solve_date: string
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
    redirect: string
}



export async function submitSolve(
    data: SolveData,
    endpoint = "submit-solve",
): Promise<UpdateSolveResponse> {

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

    const res = await fetch(`${BACKEND}/${endpoint}`, {
        method: 'POST',
        body: formData,
        credentials: 'include',
    })
    if (!res.ok) {
        const body = await res.text()
        throw new Error(body.trim() || `Request failed with status ${res.status}`)
    }

    const redirectUrl = new URL(res.url)
    return {
        solve_id: Number(redirectUrl.searchParams.get("id")),
        redirect: `${redirectUrl.pathname}${redirectUrl.search}${redirectUrl.hash}`
    }
}




// ------------------------------------------
// Authentication functions
// ------------------------------------------



export async function updateUser(data: UpdateUserData): Promise<void> {
    const formData = new FormData()
    const appendIfDefined = (key: string, value: string | number | boolean | undefined) => {
        if (value !== undefined) {
            formData.append(key, value.toString())
        }
    }

    appendIfDefined("id", data.id)
    appendIfDefined("name", data.name?.trim() || undefined)
    appendIfDefined("email", data.email?.trim() || undefined)
    appendIfDefined("discord", data.discord)
    appendIfDefined("moderator_notes", data.moderator_notes?.trim() || undefined)
    appendIfDefined("moderator", data.moderator)
    appendIfDefined("dummy", data.dummy)
    appendIfDefined("audit_log_comment", data.audit_log_comment?.trim() || undefined)

    const response = await fetch(`${BACKEND}/update-user`, {
        method: "POST",
        body: formData,
        credentials: "include",
    })
    if (!response.ok) {
        throw new Error(`Unable to update user (${response.status})`)
    }
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

export type SignInEmailRequest = {
    email: string,
    turnstile_response?: string,
    redirect?: string
}

async function requestOtp(path: string, formData: FormData, authType: string): Promise<OtpResponse|null> {
    try {
        const res = await fetch(`${BACKEND}/${path}`, {
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
            auth_type: authType,
        }
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function requestOtpDiscord(data: SignInDiscordRequest): Promise<OtpResponse|null> {
    const formData = new FormData()
    formData.append("username", data.username)
    if (data.turnstile_response !== undefined) {
        formData.append("cf-turnstile-response", data.turnstile_response)
    }
    if (data.redirect !== undefined) {
        formData.append("redirect", data.redirect)
    }

    return requestOtp("request-otp-discord", formData, "DiscordOtp")
}

export async function requestOtpEmail(data: SignInEmailRequest): Promise<OtpResponse|null> {
    const formData = new FormData()
    formData.append("email", data.email)
    if (data.turnstile_response !== undefined) {
        formData.append("cf-turnstile-response", data.turnstile_response)
    }
    if (data.redirect !== undefined) {
        formData.append("redirect", data.redirect)
    }

    return requestOtp("request-otp-email", formData, "EmailOtp")
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

export async function updateName(targetId: number|undefined, new_name?: string) {
    try {
        const formData = new FormData()
        if (targetId !== undefined) {
            formData.append("target_user_id", targetId.toString())
        }

        if(new_name?.trim()) {
            formData.append("new_name", new_name.trim())
        }

        formData.append("redirect", "/settings")

        const res = await fetch(`${BACKEND}/update-name`, {
            method: "POST",
            body: formData,
            credentials: 'include',
        })

        if(!res.ok) {
            const body = await res.text()
            throw new Error(`HTTP error! Status: ${res.status} ${body}`)
        }

        const redirectedUrl = new URL(res.url)
        return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
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

export async function signOutEverywhere() {
    try {
        const res = await fetch(`${BACKEND}/sign-out-everywhere`, {
            credentials: "include",
        })
        if (! res.ok) return null

        const redirectedUrl = new URL(res.url)
        return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
    } catch(err) {
        console.log("error", err)
    }
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


// ------------------------------------------
// Moderator functions
// ------------------------------------------

export async function getGeneralAuditLog() {
    try {
        const res = await fetch(`${BACKEND}/json/audit-log/general`, {
            credentials: 'include',
        })
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export type RenderedAuditLogEntry = {
    timestamp: string,
    editor_name: string,
    editor_url: string,
    description: string
}

export async function getUserAuditLog(userId: number) {
    try {
        const res = await fetch(`${BACKEND}/json/audit-log/user?id=${userId}`, {
            credentials: 'include',
        })
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function getSolveAuditLog(solveId: number) {
    try {
        const res = await fetch(`${BACKEND}/json/audit-log/solve?id=${solveId}`, {
            credentials: 'include',
        })
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function getAllUsers() {
    try {
        const res = await fetch(`${BACKEND}/json/users`, {
            credentials: 'include',
        })
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}

export async function getPendingSubmissions() {
    try {
        const res = await fetch(`${BACKEND}/json/pending-submissions`, {
            credentials: 'include',
        })
        if (! res.ok) return null
        return res.json()
    } catch(err) {
        console.log("error fetching data", err)
    }
    return null
}


export async function verifySolve(
    solveId: number,
    field: "speed" | "fmc",
    action: "accept" | "reject" | "unverify",
    comment: string,
): Promise<boolean> {
    const formData = new FormData()
    formData.append("solve_id", solveId.toString())
    formData.append(field, action)
    formData.append("audit_log_comment", comment)

    const response = await fetch(`${BACKEND}/verify-solve`, {
        method: "POST",
        body: formData,
        credentials: "include",
    })

    return response.ok
}

export async function autoVerification(): Promise<boolean> {
    const formData = new FormData()
    const response = await fetch(`${BACKEND}/request-autoverify-all-pending`, {
        method: "POST",
        body: formData,
        credentials: "include",
    })

    return response.ok
}