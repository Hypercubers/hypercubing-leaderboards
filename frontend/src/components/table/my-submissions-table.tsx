import { html_render_date, html_render_time } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { FullSolve } from "@/lib/backend"
import { Link, useNavigate } from "react-router-dom"
import ProgramIcon from "../icon/program-icon"
import { Check, Timer, X } from "lucide-react"

interface solveData {
    FullSolves?: FullSolve[]
}

function getIconFromVerificationStatus(status: boolean|undefined) {
    if (status == undefined) {
        return  <Timer className="size-4 shrink-0 text-yellow-500"/>
    } else if (status) {
        return <Check className="size-4 shrink-0 text-green-500"/>
    } else {
        return <X className="size-4 shrink-0 text-red-500"/>
    }

}

/**
 * Table that renders My Submissions FullSolve arrays with complete formatting, icons, links, etc.
 * @returns
 */
function MySubmissionsTable({FullSolves}: solveData) {
    const navigate = useNavigate()
    return (
        <Table className="table-fixed md:table-auto">
            <TableHeader>
                <TableRow >
                    <TableHead>Puzzle</TableHead>
                    <TableHead className="text-right">Time</TableHead>
                    <TableHead className="text-right">Move count</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead className="hidden md:table-cell">Program</TableHead>
                </TableRow>
            </TableHeader>
            {FullSolves &&
            <TableBody>
                {FullSolves.length > 0 ? FullSolves.map((s) => (
                    <TableRow className="*:p-2" key={s.id} onClick={() => navigate(`/solve?id=${s.id}`)}>
                        <TableCell className="text-sidebar-primary hover:underline truncate">
                            <Link to={`/puzzle?id=${s.puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                {s.puzzle.name}
                            </Link>
                        </TableCell>
                        <TableCell>
                            <div className="inline-flex items-center justify-end gap-2 w-full">
                                {s.speed_cs && html_render_time(s.speed_cs)}
                                {s.speed_cs && getIconFromVerificationStatus(s.speed_verified)}
                            </div>
                        </TableCell>
                        <TableCell>
                            <div className="inline-flex items-center justify-end gap-2 w-full">
                                {s.move_count && s.move_count.toString()}
                                {s.move_count && getIconFromVerificationStatus(s.fmc_verified)}
                            </div>
                        </TableCell>
                        <TableCell className="text-center">{html_render_date(s.solve_date)}</TableCell>
                        <TableCell className="hidden md:inline-flex items-center"><ProgramIcon abbr={s.program.abbr}/>&nbsp;{s.program.abbr}</TableCell>
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
