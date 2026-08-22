import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table"
import type { FullSolve } from "@/lib/backend"
import { useNavigate } from "react-router-dom"
import ProgramIcon from "./icon/program-icon"
import { Check, Timer, X } from "lucide-react"

interface solveData {
    FullSolves?: FullSolve[]
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
                    <TableHead className="text-right">Time</TableHead>
                    <TableHead className="text-right">Move count</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead>Program</TableHead>
                </TableRow>
            </TableHeader>
            {FullSolves &&
            <TableBody>
                {FullSolves.length > 0 ? FullSolves.map((s) => (
                    <TableRow key={s.id} onClick={() => navigate(`/solve?id=${s.id}`)}>
                        <TableCell>{s.puzzle.name}</TableCell>
                        <TableCell>
                            <div className="inline-flex items-center justify-end gap-2 w-full">
                                {s.speed_cs && html_render_time(s.speed_cs)}
                                {s.speed_cs && (s.speed_verified==undefined ? <Timer className="text-yellow-500"/> : (s.speed_verified ? <Check className="text-green-500"/> : <X className="text-red-500"/>))}
                            </div>
                        </TableCell>
                        <TableCell>
                            <div className="inline-flex items-center justify-end gap-2 w-full">
                                {s.move_count && s.move_count.toString()}
                                {s.move_count && (s.fmc_verified==undefined ? <Timer className="text-yellow-500"/> : (s.fmc_verified ? <Check className="text-green-500"/> : <X className="text-red-500"/>))}
                            </div>
                        </TableCell>
                        <TableCell className="text-center">{html_render_date(s.solve_date)}</TableCell>
                        <TableCell className="inline-flex items-center"><ProgramIcon abbr={s.program.abbr}/>&nbsp;{s.program.abbr}</TableCell>
                    </TableRow>
                ))
                :
                    <>
                    </>
                }
            </TableBody>
        }
        </Table>
    )
}

export default MySubmissionsTable
