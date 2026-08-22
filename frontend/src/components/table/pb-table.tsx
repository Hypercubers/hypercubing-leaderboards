import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { PB } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "../icon/program-icon"
import RankIcon from "../icon/rank-icon"

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
                        <TableHead className="text-right">Rank</TableHead>
                        <TableHead className="text-right">{isFmc ? "Move count" : "Time"}</TableHead>
                        <TableHead className="text-center">Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {PBs && PBs.length > 0 ? PBs.map((solve) => (
                        <TableRow onClick={() => navigate(`/solve?id=${solve[1].solve.id}`)}>
                            <TableCell className="text-sidebar-primary hover:underline">
                                <Link to={`/puzzle?id=${solve[1].solve.puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                    {solve[1].solve.puzzle.name}
                                </Link>
                            </TableCell>
                            <TableCell className="inline-flex items-center justify-end w-full">
                                <RankIcon rank={solve[1].rank}/>
                                &nbsp;{solve[1].rank}
                            </TableCell>
                            <TableCell className="text-right">
                                { isFmc ?
                                solve[1].solve.move_count :
                                solve[1].solve.speed_cs && html_render_time(solve[1].solve.speed_cs)}
                            </TableCell>
                            <TableCell className="text-center">{html_render_date(solve[1].solve.solve_date)}</TableCell>
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
