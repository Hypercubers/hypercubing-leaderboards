import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { PB } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "../icon/program-icon"

interface solveData {
    PBs?: PB[],
    isFmc: boolean,
}

/**
 * Reusable table that renders PB arrays with complete formatting, icons, links, etc.
 * @returns
 */
function PbTable({PBs, isFmc}: solveData) {
    const navigate = useNavigate()
    return (
        <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Rank</TableHead>
                        <TableHead>{isFmc ? "Move count" : "Time"}</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {PBs && PBs.length > 0 ? PBs.map((solve) => (
                        <TableRow onClick={() => navigate(`/solve?id=${solve[1].solve.id}`)}>
                            <TableCell>
                                <Link to={`/puzzle?id=${solve[1].solve.puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                    {solve[1].solve.puzzle.name}
                                </Link>
                            </TableCell>
                            <TableCell>{solve[1].rank}</TableCell>
                            <TableCell>
                                { isFmc ?
                                solve[1].solve.move_count :
                                solve[1].solve.speed_cs && html_render_time(solve[1].solve.speed_cs)}
                            </TableCell>
                            <TableCell>{html_render_date(solve[1].solve.solve_date)}</TableCell>
                            <TableCell className="inline-flex items-center"><ProgramIcon abbr={solve[1].solve.program.abbr}/>&nbsp;{solve[1].solve.program.abbr}</TableCell>
                        </TableRow>
                    ))
                    :
                    <p className="pt-4">There are no solves matching the search.</p>
                    }
                </TableBody>
            </Table>
    )
}

export default PbTable
