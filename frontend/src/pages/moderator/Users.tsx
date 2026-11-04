import Header from "@/components/header"
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table"



function Users() {
    return (
        <>
            <Header PageTitle="User accounts"/>
            <p>BE VERY CAREFUL ON THIS PAGE. THERE IS NO UNDO BUTTON.</p>
            <p>If you need to delete any of these, contact a database administrator.</p>

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
                    </TableRow>
                </TableHeader>
            </Table>
        </>
    )
}

export default Users