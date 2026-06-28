import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger, } from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { ChevronDown } from "lucide-react"

import { useEffect, useState } from "react"

function User() {

    const [data, setData] = useState("Username")
    const [loggedIn, setLoggedIn] = useState(true)

    useEffect(() => {
        getName()
    })

    async function getName() {
        fetch('http://localhost:3000/hello')
        // .then((res) => console.log(res))
        .then((res) => res.json())
        .then((data) => setData(data.text))
        .catch((err) => {
            console.log("error fetching data", err)
        })
    }


    return (

        <div className="ml-auto">
            { loggedIn ?
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button onClick={getName} variant="outline">{data} <ChevronDown/></Button>
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
