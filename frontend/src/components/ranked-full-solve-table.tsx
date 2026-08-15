import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table"
import type { RankedFullSolve } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "./icon/program-icon"

interface solveData {
    RankedSolves?: RankedFullSolve[],
    isFmc: boolean,
}

/**
 * Reusable table that renders RankedFullSolve arrays with complete formatting, icons, links, etc.
 * @returns
 */
function RankedFullSolveTable({RankedSolves, isFmc}: solveData) {
    const navigate = useNavigate()
    return (
        <Table>
            <TableHeader>
                <TableRow >
                    <TableHead>Rank</TableHead>
                    <TableHead>Solver</TableHead>
                    <TableHead>{isFmc ? "Move count" : "Time"}</TableHead>
                    <TableHead>Date</TableHead>
                    <TableHead>Program</TableHead>
                </TableRow>
            </TableHeader>
            {RankedSolves &&
            <TableBody>
                {RankedSolves.length > 0 ? RankedSolves.map((s) => (
                    <TableRow onClick={() => navigate(`/solve?id=${s.solve.id}`)}>
                        <TableCell>{s.rank}</TableCell>
                        <TableCell>
                            <Link to={`/solver?id=${s.solve.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                {s.solve.solver.name}
                            </Link>
                        </TableCell>
                        <TableCell>
                            { isFmc ?
                            s.solve.move_count && s.solve.move_count :
                            s.solve.speed_cs && html_render_time(s.solve.speed_cs)}
                        </TableCell>
                        <TableCell>{html_render_date(s.solve.solve_date)}</TableCell>
                        <TableCell className="inline-flex items-center"><ProgramIcon abbr={s.solve.program.abbr}/>&nbsp;{s.solve.program.abbr}</TableCell>
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

export default RankedFullSolveTable
