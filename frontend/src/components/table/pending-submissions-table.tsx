import type { FullSolve } from "@/lib/backend"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import { html_render_date, html_render_time } from "@/lib/utils"
import ProgramIcon from "../icon/program-icon"
import { Check, Timer, X } from "lucide-react"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "../ui/tooltip"


interface PendingSubmissionsTableProps {
    submissions?: FullSolve[]
}

function getIconFromVerificationStatus(status: boolean|undefined) {
    if (status == undefined) {
        return  (
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                        <Timer className="size-4 shrink-0 text-yellow-500"/>
                    </TooltipTrigger>
                    <TooltipContent>
                        Awaiting verification
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        )
    } else if (status) {
        return  (
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                        <Check className="size-4 shrink-0 text-green-500"/>
                    </TooltipTrigger>
                    <TooltipContent>
                        Accepted
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        )
    } else {
        return  (
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                        <X className="size-4 shrink-0 text-red-500"/>
                    </TooltipTrigger>
                    <TooltipContent>
                        Rejected
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        )
    }
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
                                <div className="inline-flex items-center justify-end gap-2 w-full">
                                    {s.speed_cs && html_render_time(s.speed_cs)}
                                    {s.speed_cs && getIconFromVerificationStatus(s.speed_verified)}
                                </div>
                            </TableCell>
                            <TableCell className="text-right">
                                <div className="inline-flex items-center justify-end gap-2 w-full">
                                    {s.move_count && s.move_count.toString()}
                                    {s.move_count && getIconFromVerificationStatus(s.fmc_verified)}
                                </div>
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