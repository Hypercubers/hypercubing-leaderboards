import { useEffect, useState, type ReactNode } from "react"
import { AuthContext } from "@/lib/auth-context"
import { getCurrentUser, signOut, signOutEverywhere, type SelfInfoResponse } from "@/lib/backend"

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<SelfInfoResponse | null | undefined>(undefined)

    async function refreshUser() {
        const nextUser = await getCurrentUser()
        setUser(nextUser)
        return nextUser
    }

    async function handleSignOut() {
        const wasSignedOut = await signOut()
        if (wasSignedOut) {
            setUser(null)
        }
        return wasSignedOut
    }

    async function handleSignOutEverywhere() {
        const redirect = await signOutEverywhere()
        setUser(null)
        return (redirect ?? "/")
    }

    useEffect(() => {
        let isCurrent = true

        async function loadUser() {
            const nextUser = await getCurrentUser()
            if (!isCurrent) {
                return
            }

            setUser(nextUser)
        }

        void loadUser()

        return () => {
            isCurrent = false
        }
    }, [])

    return (
        <AuthContext.Provider value={{ user, refreshUser, signOut: handleSignOut, signOutEverywhere: handleSignOutEverywhere }}>
            {children}
        </AuthContext.Provider>
    )
}
