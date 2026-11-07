import { useEffect, useState, type ReactNode } from "react"
import { AuthContext } from "@/lib/auth-context"
import { getCurrentUser, signOut, signOutEverywhere, type SelfInfoResponse } from "@/lib/backend"

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<SelfInfoResponse | null | undefined>(undefined)
    const [status, setStatus] = useState<"loading" | "unauthenticated" | "authenticated">("loading")

    async function refreshUser() {
        const nextUser = await getCurrentUser()
        setUser(nextUser)
        setStatus(nextUser === null ? "unauthenticated" : "authenticated")
        return nextUser
    }

    async function handleSignOut() {
        const wasSignedOut = await signOut()
        if (wasSignedOut) {
            setUser(null)
            setStatus("unauthenticated")
        }
        return wasSignedOut
    }

    async function handleSignOutEverywhere() {
        const redirect = await signOutEverywhere()
        setUser(null)
        setStatus("unauthenticated")
        return (redirect ?? "/")
    }

    useEffect(() => {
        let isCurrent = true

        async function loadUser() {
            try {
                const nextUser = await getCurrentUser()
                if (!isCurrent) {
                    return
                }

                setUser(nextUser)
                setStatus(nextUser === null ? "unauthenticated" : "authenticated")
            } catch {
                if (!isCurrent) {
                    return
                }

                setUser(null)
                setStatus("unauthenticated")
            }
        }

        void loadUser()

        return () => {
            isCurrent = false
        }
    }, [])

    return (
        <AuthContext.Provider value={{ status, user, refreshUser, signOut: handleSignOut, signOutEverywhere: handleSignOutEverywhere }}>
            {children}
        </AuthContext.Provider>
    )
}
