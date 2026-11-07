import { useAuth } from "@/lib/auth-context"
import { Navigate, Outlet } from "react-router-dom"

function RequireAuth() {
    const { user } = useAuth()

    if (user === undefined || user === null) {
        return <Navigate to="/signin" replace />
    }

    return <Outlet />
}

export default RequireAuth
