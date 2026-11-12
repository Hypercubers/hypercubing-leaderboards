import { useAuth } from "@/lib/auth-context"
import { Outlet } from "react-router-dom"
import Error from "@/pages/Error"

function RequireModerator() {
    const { status, user } = useAuth()

    if (status === "loading") {
        return null
    }

    if (status === "unauthenticated" || user === null || user === undefined) {
        return <Error code="404" msg="Not Found. The page you are looking for does not exist."/>
    }

    if (!user.moderator) {
        return <Error code="404" msg="Not Found. The page you are looking for does not exist."/>
    }

    return <Outlet />
}

export default RequireModerator
