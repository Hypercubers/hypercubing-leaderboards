import { getJson, toFormData, sendRequest } from "./client.ts"
import type { User } from "./users.ts"

/*
 * Types and functions related to authentication
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

// GETTERS

async function requestOtp(path: string, fields: Record<string, string | undefined>, authType: string): Promise<OtpResponse|null> {
    const res = await sendRequest(`/${path}`, {
        method: "POST",
        body: toFormData(fields),
        credentials: "include"
    })

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
}

export async function requestOtpDiscord(data: SignInDiscordRequest): Promise<OtpResponse|null> {
    return requestOtp("request-otp-discord", {
        username: data.username,
        "cf-turnstile-response": data.turnstile_response,
        redirect: data.redirect
    }, "DiscordOtp")
}

export async function requestOtpEmail(data: SignInEmailRequest): Promise<OtpResponse|null> {
    return requestOtp("request-otp-email", {
        email: data.email,
        "cf-turnstile-response": data.turnstile_response,
        redirect: data.redirect
    }, "EmailOtp")
}

export async function submitOtpRequest(data: SubmitOtpRequest): Promise<string|null> {
    const res = await sendRequest("/submit-otp", {
        method: "POST",
        body: toFormData({
            device_code: data.device_code,
            otp: data.otp
        }),
        credentials: "include"
    })
    if (!res.ok) return null
    if (!res.url) return null
    const redirectedUrl = new URL(res.url)
    return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
}

export async function getCurrentUser(): Promise<SelfInfoResponse|null> {
    return getJson<SelfInfoResponse|null>(`/self-info`, { credentials: "include" })
}

export async function signOut(): Promise<boolean> {
    const res = await sendRequest("/sign-out", { credentials: "include" })
    return res.ok
}

export async function signOutEverywhere(): Promise<string|null> {
    const res = await sendRequest("/sign-out-everywhere", { credentials: "include" })
    if (!res.ok) return null
    if (!res.url) return null
    const redirectedUrl = new URL(res.url)
    return `${redirectedUrl.pathname}${redirectedUrl.search}${redirectedUrl.hash}`
}