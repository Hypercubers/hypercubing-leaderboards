import { EllipsisVertical, ListTodo, Pencil, UserRound } from "lucide-react"
import { Button } from "./ui/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger } from "./ui/dropdown-menu"
import type { User } from "@/lib/backend"
import { useNavigate } from "react-router-dom"
import EditUser from "./add-update-user"
import { Dialog, DialogContent } from "./ui/dialog"
import { useState } from "react"

interface UserActionsProps {
    user: User
}

function UserActions({ user }: UserActionsProps) {
    const navigate = useNavigate()
    const [editUserOpen, setEditUserOpen] = useState(false)

    return (
        <>
        <Dialog open={editUserOpen} onOpenChange={setEditUserOpen}>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button variant="secondary"><EllipsisVertical/></Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuLabel>{user.name ?? user.id}</DropdownMenuLabel>
                    <DropdownMenuItem onClick={() => navigate(`/solver?id=${user.id}`)}><UserRound/> Solver page</DropdownMenuItem>
                    <DropdownMenuItem><ListTodo/> Submissions</DropdownMenuItem>
                    <DropdownMenuItem onSelect={() => setEditUserOpen(true)}><Pencil/> Edit user</DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
            <DialogContent>
                <EditUser add={false} fillValues={user}/>
            </DialogContent>
        </Dialog>
        </>
    )

}

export default UserActions