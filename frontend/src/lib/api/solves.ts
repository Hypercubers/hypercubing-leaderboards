import { category_query_to_url_params } from "../utils.ts"
import { getJson, toFormData, sendRequest } from "./client.ts"
import type { Program, Puzzle, Variant } from "./puzzles.ts"
import type { PublicUser } from "./users.ts"

/*
 * Types and functions related to solves and records
*/

// TYPES

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
    computer_assisted: boolean
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
    computer_assisted: boolean
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

export type WorldRecord = [Event, FullSolve]

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
    distinct: boolean,
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

export type Distinct = [number, PublicUser, string]

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


// GETTERS

export async function getWorldRecords(query: CategoryQuery | undefined): Promise<WorldRecord[]|null> {
    return getJson<WorldRecord[]>(`/json/all-puzzles-leaderboard?${category_query_to_url_params(query)}`)
}

export async function getDistinctRecords(): Promise<Distinct[]|null> {
    return getJson<Distinct[]>("/json/distinct_puzzles_leaderboards")
}

export async function getRecordHistory(id: number, query: CategoryQuery | undefined): Promise<RankedFullSolve[]|null> {
    return getJson<RankedFullSolve[]>(`/json/recordhistory?id=${id}&${category_query_to_url_params(query)}`)
}

export async function getSolve(id: number): Promise<FullSolve|null> {
    return getJson<FullSolve>(`/json/solve?id=${id}`)
}

export async function getSolveLogs(id: number): Promise<string|null> {
    return getJson<string>(`/json/solve-logs?id=${id}`)
}

export async function getPuzzleSolves(id: number, query: CategoryQuery | undefined): Promise<RankedFullSolve[]|null> {
    return getJson<RankedFullSolve[]>(`/json/puzzle?id=${id}&${category_query_to_url_params(query)}`)
}

export async function getUserPbs(id: number, query: CategoryQuery | undefined): Promise<PB[]|null> {
    return getJson<PB[]>(`/json/user/pbs?id=${id}&${category_query_to_url_params(query)}`)
}

// id was optional for some reason
export async function getUserSubmissions(id: number|null): Promise<FullSolve[]|null> {
    return getJson<FullSolve[]>(`/json/user/submissions?id=${id}`)
}

// UPDATERS

export async function submitSolve(data: SolveData): Promise<UpdateSolveResponse> {
    const res = await sendRequest("/submit-solve", {
        method: "POST",
        body: toFormData({
            ...data, solve_date: data.solve_date.toString()
        }),
        credentials: "include"
    })
    const url = new URL(res.url)
    return {
        solve_id: Number(url.searchParams.get("id")),
        redirect: `${url.pathname}${url.search}${url.hash}`
    }
}

// MODERATOR FUNCTIONS

export async function verifySolve(
    solveId: number,
    field: "speed" | "fmc",
    action: "accept" | "reject" | "unverify",
    comment: string,
): Promise<boolean> {
    const res = await sendRequest("/verify-solve", {
        method: "POST",
        body: toFormData({
            solve_id: solveId.toString(),
            [field]: action,
            audit_log_comment: comment
        }),
        credentials: "include"
    })
    return res.ok
}

export async function autoVerification(): Promise<boolean> {
    const res = await sendRequest("/request-autoverify-all-pending", {
        method: "POST",
        body: new FormData(),
        credentials: "include"
    })
    return res.ok
}

export async function getPendingSubmissions(): Promise<FullSolve[]|null> {
    return getJson<FullSolve[]>("/json/pending-submissions", { credentials: "include" })
}