import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger, } from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { ChevronDown, CirclePlus, LogOut, UserRound } from "lucide-react"
import { useNavigate } from "react-router-dom"
import { useAuth } from "@/lib/auth-context"

function User() {
    const navigate = useNavigate()
    const { user, signOut } = useAuth()

    const displayName = user?.name ?? user?.discord_nickname ?? user?.discord_username ?? user?.email ?? "User"

    async function handleSignOut() {
        const wasSignedOut = await signOut()
        if (wasSignedOut) {
            navigate("/")
        }
    }


    return (

        <div className="ml-auto">
            { user === undefined ?
            <Button variant="outline" disabled>Loading...</Button>
            : user ?
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="outline">{displayName} <ChevronDown/></Button>
                </DropdownMenuTrigger>

                <DropdownMenuContent>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>My Account</DropdownMenuLabel>
                        <DropdownMenuItem onClick={() => navigate(`/solver?id=${user.id}`)}><UserRound/> Profile</DropdownMenuItem>
                        <DropdownMenuItem onClick={() => navigate("/submit-solve")}><CirclePlus/> Submit solve</DropdownMenuItem>
                        <DropdownMenuItem variant="destructive" onClick={() => void handleSignOut()}><LogOut/> Sign out</DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>

            </DropdownMenu>

        :
        <Button variant="secondary" onClick={() => navigate("/signin")}>Sign In</Button>
        }
        </div>
    )
}

export default User
