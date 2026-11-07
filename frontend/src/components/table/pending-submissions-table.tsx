import { Table, TableHead, TableHeader, TableRow } from "../ui/table"



function PendingSubmissionsTable() {
    return (
        <Table className="table-fixed md:table-auto">
            <TableHeader>
                <TableRow >
                    <TableHead>Puzzle</TableHead>
                    <TableHead>Solver</TableHead>
                    <TableHead className="text-right">Time</TableHead>
                    <TableHead className="text-right">Move count</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead className="hidden md:table-cell">Program</TableHead>
                </TableRow>
            </TableHeader>
        </Table>
    )
}

export default PendingSubmissionsTable