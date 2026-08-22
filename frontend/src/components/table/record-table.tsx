import { html_render_date, html_render_time, puz_name } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import ProgramIcon from "../icon/program-icon"
import { Link, useNavigate } from "react-router-dom"
import type { Record } from "@/lib/backend"



interface solveData {
    Records?: Record[],
    isFmc: boolean,
}

function RecordTable({Records, isFmc}: solveData) {
    const navigate = useNavigate()
    return (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>Puzzle</TableHead>
                    <TableHead>Record Holder</TableHead>
                    <TableHead className="text-right">{isFmc ? "Move count" : "Time"}</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead>Program</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {Records && Records.length > 0 ? Records.map((rec) => (
                    <TableRow onClick={() =>  navigate(`/solve?id=${rec[1].id}`)}>
                        <TableCell className="text-sidebar-primary hover:underline">
                            <Link to={`/puzzle?id=${rec[1].puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                {`${puz_name(rec[0])}`}
                            </Link>
                        </TableCell>

                        <TableCell className="text-sidebar-primary hover:underline">
                            <Link to={`/solver?id=${rec[1].solver.id}`} onClick={(e) => e.stopPropagation()}>
                                {rec[1].solver.name}
                            </Link>
                        </TableCell>

                        <TableCell className="text-right">
                            {isFmc ?
                            rec[1].move_count && rec[1].move_count :
                            rec[1].speed_cs && html_render_time(rec[1].speed_cs)}
                        </TableCell>

                        <TableCell className="text-center">{html_render_date(rec[1].solve_date)}</TableCell>

                        <TableCell className="inline-flex items-center"><ProgramIcon abbr={rec[1].program.abbr}/>&nbsp;{rec[1].program.abbr}</TableCell>
                    </TableRow>
                ))
                :
                <>
                    <p className="pt-4">No world records found.</p>
                </>
                }
            </TableBody>
        </Table>

    )
}

export default RecordTable
