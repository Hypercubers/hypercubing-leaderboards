import { createContext, useContext } from "react"
import type { SelfInfoResponse } from "@/lib/backend"

export type AuthContextValue = {
    // null: logged out
    // undefined: loading / still checking
    user: SelfInfoResponse | null | undefined
    refreshUser: () => Promise<SelfInfoResponse | null>
    signOut: () => Promise<boolean>
    signOutEverywhere: () => Promise<string>
}

export const AuthContext = createContext<AuthContextValue | null>(null)

export function useAuth() {
    const context = useContext(AuthContext)
    if (context === null) {
        throw new Error("useAuth must be used within an AuthProvider")
    }

    return context
}
