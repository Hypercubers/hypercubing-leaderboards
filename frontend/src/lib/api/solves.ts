import { category_query_to_url_params } from "../utils.ts"
import { getJson, toFormData, sendRequest } from "./client.ts"
import type { Program, Puzzle, Variant } from "./puzzles.ts"
import type { PublicUser } from "./users.ts"

/*
 * FullSolve, SolveData, getSolve, submitSolve, verifySolve, etc. types and get/update functions
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