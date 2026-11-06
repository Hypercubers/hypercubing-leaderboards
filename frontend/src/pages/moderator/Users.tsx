import EditUser from "@/components/add-update-user"
import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import UserActions from "@/components/user-actions"
import { getAllUsers, type User } from "@/lib/backend"
import { Eye, ListTodo, Pencil, Shield, UserRoundX } from "lucide-react"
import { useEffect, useState } from "react"
import { Link, useNavigate } from "react-router-dom"


function Users() {
    const navigate = useNavigate()

    const [users, setUsers] = useState<User[]>([])

    useEffect(() => {
        getAllUsers().then(setUsers)
    }, [])

    return (
        <>
            <Header PageTitle="User accounts"/>
            <p>BE VERY CAREFUL ON THIS PAGE. THERE IS NO UNDO BUTTON.</p>
            <p>If you need to delete any of these, contact a database administrator.</p>

            {/* <EditUser display="button" add={true}/> */}

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>ID</TableHead>
                        <TableHead>Solves</TableHead>
                        <TableHead>Account</TableHead>
                        <TableHead>Name</TableHead>
                        <TableHead>Email</TableHead>
                        <TableHead>Discord ID</TableHead>
                        <TableHead>Moderator notes</TableHead>
                        {/* <TableHead>Is moderator</TableHead>
                        <TableHead>Is dummy</TableHead> */}
                        <TableHead>Edit</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {users && users.sort((a, b) => a.id - b.id).map((user) => (
                        <TableRow className="*:p-2" key={user.id}>
                            <TableCell><Link className="text-sidebar-primary hover:underline" to={`/solver?id=${user.id}`}>{user.id}</Link></TableCell>
                            <TableCell><Link className="text-sidebar-primary hover:underline" to={`/solver?id=${user.id}`}>{<ListTodo className="size-4 shrink-0"/>}</Link></TableCell>
                            <TableCell>
                                <div className="flex items-center gap-2">
                                    {user.moderator &&
                                        <TooltipProvider>
                                            <Tooltip>
                                                <TooltipTrigger asChild>
                                                    <Shield className="size-4 shrink-0"/>
                                                </TooltipTrigger>
                                                <TooltipContent>
                                                    Moderator account
                                                </TooltipContent>
                                            </Tooltip>
                                        </TooltipProvider>
                                    }
                                    {user.dummy &&
                                        <TooltipProvider>
                                            <Tooltip>
                                                <TooltipTrigger asChild>
                                                    <UserRoundX className="size-4 shrink-0"/>
                                                </TooltipTrigger>
                                                <TooltipContent>
                                                    Dummy account
                                                </TooltipContent>
                                            </Tooltip>
                                        </TooltipProvider>
                                    }
                                </div>
                            </TableCell>
                            <TableCell>{user.name}</TableCell>
                            <TableCell>{user.email}</TableCell>
                            <TableCell>{user.discord_id}</TableCell>
                            <TableCell>
                                {user.moderator_notes.length > 0 ? 
                                <Popover>
                                    <PopoverTrigger asChild>
                                        <Button variant="secondary"><Eye/> View</Button>
                                    </PopoverTrigger>
                                    <PopoverContent align="center">
                                        {user.moderator_notes}
                                    </PopoverContent>
                                </Popover>
                                : <></>
                                }
                            </TableCell>
                            <TableCell>
                                <Dialog>
                                    <DialogTrigger asChild>
                                        <Button variant="secondary"><Pencil/> Edit</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <EditUser add={false} fillValues={user}/>
                                    </DialogContent>
                                </Dialog>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </>
    )
}
                               
export default Users