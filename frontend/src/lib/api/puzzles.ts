import { getJson, toFormData, sendRequest } from "./client.ts"

/*
 * Puzzle, Variant, Program, etc. types and get/update functions
*/

// TYPES

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
    abbr: string,
    material_by_default: boolean,
    primary_filters: boolean,
    primary_macros: boolean,
}

export type CombinedVariant = {
    name: string,
    variant_abbr?: string
    program?: string
}

export type Program = {
    id: number,
    name: string,
    abbr: string,
    material: boolean
}

export type UpdateProgramData = {
    id?: number,
    name: string,
    abbr: string,
    material: boolean,
    audit_log_comment?: string,
}

export type UpdateVariantData = {
    id?: number,
    name: string,
    prefix: string,
    suffix: string,
    abbr: string,
    material_by_default: boolean,
    primary_filters: boolean,
    primary_macros: boolean,
    audit_log_comment?: string,
}

export type UpdatePuzzleData = {
    id?: number,
    name: string,
    hsc_id?: string,
    autoverifiable: boolean,
    primary_filters: boolean,
    primary_macros: boolean,
    audit_log_comment?: string,
}

// GETTERS

export async function getVariants(): Promise<Variant[]|null> {
    return getJson<Variant[]>("/json/variants")
}

export async function getCombinedVariants(id: number): Promise<CombinedVariant[]|null> {
    return getJson<CombinedVariant[]>(`/json/combinedvariants?id=${id}`)
}

export async function getPrograms(): Promise<Program[]|null> {
    return getJson<Program[]>("/json/programs")
}

export async function getPuzzles(): Promise<Puzzle[]|null> {
    return getJson<Puzzle[]>("/json/puzzles")
}

export async function getPuzzleInfo(id: number): Promise<Puzzle> {
    return getJson<Puzzle>(`/json/puzzleinfo?id=${id}`)
}

// UPDATERS

export async function updateProgram(data: UpdateProgramData) {
    await sendRequest("/update-program", {
        method: "POST",
        body: toFormData({
            id: data.id,
            name: data.name.trim(),
            abbr: data.abbr.trim(),
            material: data.material,
            audit_log_comment: data.audit_log_comment?.trim() || undefined

        }), credentials: "include"
    })
}

export async function updateVariant(data: UpdateVariantData) {
    await sendRequest("/update-variant", {
        method: "POST",
        body: toFormData({
            id: data.id,
            name: data.name.trim(),
            prefix: data.prefix.trim(),
            suffix: data.suffix.trim(),
            abbr: data.abbr.trim(),
            material_by_default: data.material_by_default,
            primary_filters: data.primary_filters,
            primary_macros: data.primary_macros,
            audit_log_comment: data.audit_log_comment?.trim() || undefined
        }), credentials: "include"
    })
}

export async function updatePuzzle(data: UpdatePuzzleData) {
    await sendRequest("/update-puzzle", {
        method: "POST",
        body: toFormData({
            id: data.id,
            name: data.name.trim(),
            hsc_id: data.hsc_id?.trim() || undefined,
            autoverifiable: data.autoverifiable,
            primary_filters: data.primary_filters,
            primary_macros: data.primary_macros,
            audit_log_comment: data.audit_log_comment?.trim() || undefined
        }), credentials: "include"
    })
}

