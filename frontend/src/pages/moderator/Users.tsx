import EditUser from "@/components/add-update-user"
import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Table, TableBody, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { CirclePlus, Edit, Pencil } from "lucide-react"



function Users() {
    return (
        <>
            <Header PageTitle="User accounts"/>
            <p>BE VERY CAREFUL ON THIS PAGE. THERE IS NO UNDO BUTTON.</p>
            <p>If you need to delete any of these, contact a database administrator.</p>

            <EditUser add={true}/>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>ID</TableHead>
                        <TableHead>Name</TableHead>
                        <TableHead>Email</TableHead>
                        <TableHead>Discord ID</TableHead>
                        <TableHead>Moderator notes</TableHead>
                        <TableHead>Moderator</TableHead>
                        <TableHead>Dummy</TableHead>
                        <TableHead>Edit</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableHead>1</TableHead>
                        <TableHead>John Doe</TableHead>
                        <TableHead>john.doe@example.com</TableHead>
                        <TableHead>123456789</TableHead>
                        <TableHead>No issues</TableHead>
                        <TableHead>No</TableHead>
                        <TableHead>No</TableHead>
                        <TableHead>
                            <EditUser add={false} fillValues={{
                                id: 1,
                                name: "John Doe",
                                email: "john.doe@example.com",
                                discord_id: "123456789",
                                moderator_notes: "No issues",
                                moderator: false,
                                dummy: false
                            }}/>
                        </TableHead>
                    </TableRow>
                </TableBody>
            </Table>
        </>
    )
}

export default Users