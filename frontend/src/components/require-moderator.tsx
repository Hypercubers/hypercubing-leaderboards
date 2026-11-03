import { useAuth } from "@/lib/auth-context"
import { Navigate, Outlet } from "react-router-dom"
import Error from "@/pages/Error"

function RequireModerator() {
    const { user } = useAuth()

    if (user === undefined) {
        return null
    }

    if (user === null) {
        return <Navigate to="/signin" replace />
    }

    if (!user.moderator) {
        return <Error code="404" msg="Not Found. The page you are looking for does not exist."/>
    }

    return <Outlet />
}

export default RequireModerator
