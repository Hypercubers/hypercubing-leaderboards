import { getJson, toFormData, sendRequest } from "./client.ts"

/*
 * OTP flow, getCurrentUser, signOut, etc. types and get/update functions
*/ 


// TYPES

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