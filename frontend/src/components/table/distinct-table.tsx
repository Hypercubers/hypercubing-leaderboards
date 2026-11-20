import type { Distinct } from "@/lib/backend";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table";
import { Link } from "react-router-dom";
import EmptyData from "../empty-data";


interface solveData {
    Records: Distinct[],
}

/**
 *
 * Reusable table that renders Distinct arrays with complete formatting, icons, links, etc.
 */
function DistinctTable({Records}: solveData) {
    if (Records.length == 0) {
        return (
            <EmptyData word="solves" />
        )
    }
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
                {Records.map((record) => (
                    <TableRow className="*:p-2">
                        <TableCell>{record[0]}</TableCell>
                        <TableCell className="text-sidebar-primary hover:underline">
                            <Link to={`/solver?id=${record[1].id}`} onClick={(e) => e.stopPropagation()}>
                                {record[1].name || record[1].id.toString()}
                            </Link>
                        </TableCell>
                        <TableCell>{record[2]}</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    )
}

export default DistinctTable
