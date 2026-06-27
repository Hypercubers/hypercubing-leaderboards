import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger, } from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { ChevronDown } from "lucide-react"

import { useEffect, useState } from "react"

function User() {

    const [data, setData] = useState("Username")

    useEffect(() => {
        fetch('http://localhost:3000/solvers?id=5')
        .then((res) => res.json())
        .then((data) => setData(data))
        .catch((err) => {
            console.log("error fetching data", err)
        })
    })


    return (
        <div className="ml-auto">
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="outline">{data} <ChevronDown/></Button>
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
        </div>
    )
}

export default User
