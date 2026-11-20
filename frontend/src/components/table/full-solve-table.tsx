import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { FullSolve } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "../icon/program-icon"

interface solveData {
    FullSolves?: FullSolve[],
    isFmc: boolean,
}

/**
 * Reusable table that renders FullSolve arrays with complete formatting, icons, links, etc.
 * @returns
 */
function FullSolveTable({FullSolves, isFmc}: solveData) {
    const navigate = useNavigate()
    return (
        <Table className="table-fixed md:table-auto">
            <TableHeader>
                <TableRow >
                    <TableHead>Record Holder</TableHead>
                    <TableHead className="text-right">{isFmc ? "Move count" : "Time"}</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead className="hidden md:table-cell">Program</TableHead>
                </TableRow>
            </TableHeader>
            {FullSolves &&
            <TableBody>
                {FullSolves.length > 0 ? FullSolves.map((s) => (
                    <TableRow className="*:p-2" onClick={() => navigate(`/solve?id=${s.id}`)}>
                        <TableCell className="text-sidebar-primary hover:underline truncate" onClick={(e) => e.stopPropagation()}>
                            <Link to={`/solver?id=${s.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                {s.solver.name}
                            </Link>
                        </TableCell>
                        <TableCell className="text-right">
                            { isFmc ?
                            s.move_count && s.move_count :
                            s.speed_cs && html_render_time(s.speed_cs)}
                        </TableCell>
                        <TableCell className="text-center">{html_render_date(s.solve_date)}</TableCell>
                        <TableCell className="hidden md:inline-flex items-center"><ProgramIcon abbr={s.program.abbr}/>&nbsp;{s.program.abbr}</TableCell>
                    </TableRow>
                ))
                :
                    <p className="pt-4">There are no solves matching the search.</p>
                }
            </TableBody>
        }
        </Table>
    )
}

export default FullSolveTable
