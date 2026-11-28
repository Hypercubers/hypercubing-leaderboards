import { getJson } from "./client.ts"

/*
 * Types and functions related to audit logs
*/

// TYPES

export type RenderedAuditLogEntry = {
    timestamp: string,
    editor_name: string,
    editor_url: string,
    description: string
}

// GETTERS

export async function getGeneralAuditLog(): Promise<RenderedAuditLogEntry[]|null> {
    return getJson<RenderedAuditLogEntry[]|null>("/json/audit-log/general", { credentials: "include" })
}

export async function getUserAuditLog(userId: number): Promise<RenderedAuditLogEntry[]|null> {
    return getJson<RenderedAuditLogEntry[]|null>(`/json/audit-log/user?id=${userId}`, { credentials: "include" })
}

export async function getSolveAuditLog(solveId: number): Promise<RenderedAuditLogEntry[]|null> {
    return getJson<RenderedAuditLogEntry[]|null>(`/json/audit-log/solve?id=${solveId}`, { credentials: "include" })
}