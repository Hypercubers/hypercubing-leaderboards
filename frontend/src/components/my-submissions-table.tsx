import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table"
import type { FullSolve } from "@/lib/backend"
import { useNavigate } from "react-router-dom"
import ProgramIcon from "./icon/program-icon"

interface solveData {
    FullSolves?: FullSolve[],
}

/**
 * Table that renders My Submissions FullSolve arrays with complete formatting, icons, links, etc.
 * @returns
 */
function MySubmissionsTable({FullSolves}: solveData) {
    const navigate = useNavigate()
    return (
        <Table>
            <TableHeader>
                <TableRow >
                    <TableHead>Puzzle</TableHead>
                    <TableHead>Time</TableHead>
                    <TableHead>Move count</TableHead>
                    <TableHead>Date</TableHead>
                    <TableHead>Program</TableHead>
                </TableRow>
            </TableHeader>
            {FullSolves &&
            <TableBody>
                {FullSolves.length > 0 ? FullSolves.map((s) => (
                    <TableRow onClick={() => navigate(`/solve?id=${s.id}`)}>
                        <TableCell>{s.puzzle.name}</TableCell>
                        <TableCell>{s.move_count}</TableCell>
                        <TableCell>{s.speed_cs? html_render_time(s.speed_cs) : ""}</TableCell>
                        <TableCell>{html_render_date(s.solve_date)}</TableCell>
                        <TableCell className="inline-flex items-center"><ProgramIcon abbr={s.program.abbr}/>&nbsp;{s.program.abbr}</TableCell>
                    </TableRow>
                ))
                :
                    <p className="pt-4">No submissions found.</p>
                }
            </TableBody>
        }
        </Table>
    )
}

export default MySubmissionsTable
