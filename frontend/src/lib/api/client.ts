export const BACKEND = import.meta.env.VITE_BACKEND

export async function getJson<T>(path: string, init?: RequestInit): Promise<T> {
    const res = await fetch(`${BACKEND}${path}`, init)
    if (!res.ok) throw new Error(`Request to ${path} failed: ${res.status}`)
    return res.json()
}

export async function sendRequest(path: string, init?: RequestInit): Promise<Response> {
    const res = await fetch(`${BACKEND}${path}`, init)
    if (!res.ok) {
        const body = await res.text()
        throw new Error(body.trim() || `Request to ${path} failed: ${res.status}`)
    }
    return res
}


export function toFormData(fields: Record<string, string | number | boolean | File | undefined>) {
    const fd = new FormData()
    for (const [k, v] of Object.entries(fields)) {
        if (v === undefined) continue
        fd.append(k, v instanceof File ? v : v.toString())
    }
    return fd
}