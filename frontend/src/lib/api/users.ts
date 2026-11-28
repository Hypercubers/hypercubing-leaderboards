import { getJson, toFormData, sendRequest } from "./client.ts"

/*
 * Types and functions related to users
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

// GETTERS

export async function getUser(id: number): Promise<PublicUser|null> {
    return getJson<PublicUser|null>(`/json/user?id=${id}`)
}

export async function updateUser(data: UpdateUserData): Promise<void> {
    await sendRequest("/update-user", {
        method: "POST",
        body: toFormData({
            ...data, audit_log_comment: data.audit_log_comment?.trim() || undefined
        }),
        credentials: "include"
    })
}

export async function updateName(targetId: number|undefined, new_name?: string): Promise<string|null> {
    const res = await sendRequest("/update-name", {
        method: "POST",
        body: toFormData({
            target_user_id: targetId?.toString(),
            new_name: new_name?.trim(),
        }),
        credentials: "include"
    })

    const redirectedUrl = new URL(res.url)
    return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
}

export async function getAllUsers(): Promise<User[]|null> {
    return getJson<User[]>("/json/users", { credentials: "include" })
}