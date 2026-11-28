import { getJson, toFormData, sendRequest } from "./client.ts"

/*
 * PublicUser, User, getUser, getUserPbs, updateUser, etc. types and get/update functions
*/

// TYPES

export type PublicUser = {
    id: number,
    name?: string
}

export type User = {
    id: number,
    email?: string,
    discord_id?: number,
    name?: string,
    moderator: boolean,
    moderator_notes: string,
    dummy: boolean
}

export type UpdateUserData = {
    id?: number,
    name?: string,
    email?: string,
    discord?: number,
    moderator_notes?: string,
    moderator: boolean,
    dummy: boolean,
    audit_log_comment?: string,
}