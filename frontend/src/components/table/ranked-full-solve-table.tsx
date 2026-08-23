import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { RankedFullSolve } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "../icon/program-icon"
import RankIcon from "../icon/rank-icon"

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
        <Table className="table-fixed md:table-auto">
            <TableHeader>
                <TableRow >
                    <TableHead className="md:text-right">Rank</TableHead>
                    <TableHead>Solver</TableHead>
                    <TableHead className="text-right">{isFmc ? "Move count" : "Time"}</TableHead>
                    <TableHead className="hidden md:table-cell text-center">Date</TableHead>
                    <TableHead className="hidden md:table-cell">Program</TableHead>
                </TableRow>
            </TableHeader>
            {RankedSolves &&
            <TableBody>
                {RankedSolves.length > 0 ? RankedSolves.map((s) => (
                    <TableRow className="*:p-2" onClick={() => navigate(`/solve?id=${s.solve.id}`)}>
                        <TableCell className="inline-flex items-center justify-end md:w-full">
                            <RankIcon rank={s.rank}/>
                            &nbsp;{s.rank}
                        </TableCell>
                        <TableCell className="text-sidebar-primary hover:underline truncate">
                            <Link to={`/solver?id=${s.solve.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                {s.solve.solver.name}
                            </Link>
                        </TableCell>
                        <TableCell className="text-right">
                            { isFmc ?
                            s.solve.move_count && s.solve.move_count :
                            s.solve.speed_cs && html_render_time(s.solve.speed_cs)}
                        </TableCell>
                        <TableCell className="hidden md:table-cell text-center">{html_render_date(s.solve.solve_date)}</TableCell>
                        <TableCell className="hidden md:inline-flex items-center"><ProgramIcon abbr={s.solve.program.abbr}/>&nbsp;{s.solve.program.abbr}</TableCell>
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
