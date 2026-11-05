import EditUser from "@/components/add-update-user"
import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import UserActions from "@/components/user-actions"
import { getAllUsers, type User } from "@/lib/backend"
import { ShieldUser, SquareCheck, SquareDashed, UserRoundX } from "lucide-react"
import { useEffect, useState } from "react"


function Users() {

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
                        <TableHead>Status</TableHead>
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
                    {users && users.map((user) => (
                        <TableRow key={user.id}>
                            <TableCell>{user.id}</TableCell>
                            <TableCell className="flex">{(user.moderator && user.dummy) ? <><ShieldUser/><UserRoundX/></> : user.moderator ? <ShieldUser/> : user.dummy ? <UserRoundX/> : ""}</TableCell>
                            <TableCell>{user.name}</TableCell>
                            <TableCell>{user.email}</TableCell>
                            <TableCell>{user.discord_id}</TableCell>
                            <TableHead>
                                {user.moderator_notes.length > 0 ? 
                                <Popover>
                                    <PopoverTrigger asChild>
                                        <Button variant="secondary">View notes</Button>
                                    </PopoverTrigger>
                                    <PopoverContent align="center">
                                        {user.moderator_notes}
                                    </PopoverContent>
                                </Popover>
                                : <></>
                                }
                            </TableHead>
                            {/* <TableHead>{user.moderator ? <SquareCheck/> : <SquareDashed/>}</TableHead>
                            <TableHead>{user.dummy ? <SquareCheck/> : <SquareDashed/>}</TableHead> */}
                            <TableHead>
                                <UserActions user={user}/>
                            </TableHead>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </>
    )
}
                               
export default Users