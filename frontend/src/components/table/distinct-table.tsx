import type { Distinct } from "@/lib/backend";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table";
import { Link } from "react-router-dom";


interface solveData {
    Records?: Distinct[],
}

/**
 *
 * Reusable table that renders Distinct arrays with complete formatting, icons, links, etc.
 */
function DistinctTable({Records}: solveData) {
    return (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>Rank</TableHead>
                    <TableHead>Solver</TableHead>
                    <TableHead>Score</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {Records && Records?.length > 0 ? Records.map((record) => (
                    <TableRow className="*:p-2">
                        <TableCell>{record[0]}</TableCell>
                        <TableCell className="text-sidebar-primary hover:underline">
                            <Link to={`/solver?id=${record[1].id}`} onClick={(e) => e.stopPropagation()}>
                                {record[1].name || record[1].id.toString()}
                            </Link>
                        </TableCell>
                        <TableCell>{record[2]}</TableCell>
                    </TableRow>
                ))
                :
                <>
                    <p className="pt-4">No distinct puzzle records found.</p>
                </>

                }
            </TableBody>
        </Table>
    )
}

export default DistinctTable
