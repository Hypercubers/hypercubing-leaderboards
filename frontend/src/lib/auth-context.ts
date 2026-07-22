import { createContext, useContext } from "react"
import type { SelfInfoResponse } from "@/lib/backend"

export type AuthContextValue = {
    user: SelfInfoResponse | null | undefined
    refreshUser: () => Promise<SelfInfoResponse | null>
    signOut: () => Promise<boolean>
}

export const AuthContext = createContext<AuthContextValue | null>(null)

export function useAuth() {
    const context = useContext(AuthContext)
    if (context === null) {
        throw new Error("useAuth must be used within an AuthProvider")
    }

    return context
}
