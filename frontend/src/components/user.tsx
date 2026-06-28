import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger, } from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { ChevronDown } from "lucide-react"

import { useEffect, useState } from "react"

function User() {

    const [username, setUsername] = useState("Username")
    const [loggedIn, setLoggedIn] = useState(true)




    return (

        <div className="ml-auto">
            { loggedIn ?
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="outline">{username} <ChevronDown/></Button>
                </DropdownMenuTrigger>

                <DropdownMenuContent>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>My Account</DropdownMenuLabel>
                        <DropdownMenuItem>Profile</DropdownMenuItem>
                        <DropdownMenuItem>Settings</DropdownMenuItem>
                        <DropdownMenuItem>Sign out</DropdownMenuItem>
                    </DropdownMenuGroup>
                    <DropdownMenuItem>My submissions</DropdownMenuItem>
                </DropdownMenuContent>

            </DropdownMenu>

        :
        <Button>Sign in</Button>
        }
        </div>
    )
}

export default User
