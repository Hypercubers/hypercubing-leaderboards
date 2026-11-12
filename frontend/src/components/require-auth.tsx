import { useAuth } from "@/lib/auth-context"
import { Navigate, Outlet } from "react-router-dom"

function RequireAuth() {
    const { status } = useAuth()

    if (status === "loading") {
        return null
    }

    if (status === "unauthenticated") {
        return <Navigate to="/signin" replace />
    }

    return <Outlet />
}

export default RequireAuth
