import type { FullSolve } from "@/lib/backend"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import { html_render_date, html_render_time } from "@/lib/utils"
import ProgramIcon from "../icon/program-icon"


interface PendingSubmissionsTableProps {
    submissions?: FullSolve[]
}

function PendingSubmissionsTable({ submissions }: PendingSubmissionsTableProps) {
    return (
        <Table className="table-fixed md:table-auto">
            <TableHeader>
                <TableRow >
                    <TableHead>Puzzle</TableHead>
                    <TableHead>Solver</TableHead>
                    <TableHead className="text-right">Time</TableHead>
                    <TableHead className="text-right">Move count</TableHead>
                    <TableHead className="text-center">Date</TableHead>
                    <TableHead className="hidden md:table-cell">Program</TableHead>
                </TableRow>
            </TableHeader>
            {submissions && 
                <TableBody>
                    {submissions.length > 0 ? submissions.map((s) => (
                        <TableRow className="*:p-2" key={s.id}>
                            <TableCell className="text-sidebar-primary hover:underline truncate">
                                {s.puzzle.name}
                            </TableCell>
                            <TableCell className="text-sidebar-primary hover:underline truncate">
                                {s.solver.name}
                            </TableCell>
                            <TableCell className="text-right">
                                {s.speed_cs && html_render_time(s.speed_cs)}
                            </TableCell>
                            <TableCell className="text-right">
                                {s.move_count}
                            </TableCell>
                            <TableCell className="text-center">
                                {html_render_date(s.solve_date)}
                            </TableCell>
                            <TableCell className="hidden md:inline-flex items-center"><ProgramIcon abbr={s.program.abbr}/>&nbsp;{s.program.abbr}</TableCell>
                        </TableRow>
                    )) : (
                        <TableRow>
                            <TableCell colSpan={6} className="text-center">
                                No pending submissions
                            </TableCell>
                        </TableRow>
                    )}
                </TableBody>
            }
        </Table>
    )
}

export default PendingSubmissionsTable