import type { FullSolve } from "@/lib/backend"
import { Table, TableBody, TableCell, TableRow } from "../ui/table"
import { Link } from "react-router-dom"
import { html_render_date, html_render_time } from "@/lib/utils"
import { Attachment, AttachmentActions, AttachmentContent, AttachmentMedia, AttachmentTitle } from "../ui/attachment"
import { ArrowDownToLine, FileTextIcon } from "lucide-react"
import { Button } from "../ui/button"
import ProgramIcon from "../icon/program-icon"


interface SolveTableProps {
    solve: FullSolve
}

function SolveTable({ solve }: SolveTableProps) {
    return (
        <Table>
            <TableBody>
                <TableRow>
                    <TableCell>Solver</TableCell>
                    <TableCell>
                        <Link className="text-sidebar-primary hover:underline" to={`/solver?id=${solve.solver.id}`}>
                            {solve.solver.name}
                        </Link>
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>Puzzle</TableCell>
                    <TableCell>
                        <Link className="text-sidebar-primary hover:underline" to={`/puzzle?id=${solve.puzzle.id}`}>
                            {solve.puzzle.name}
                        </Link>
                    </TableCell>
                </TableRow>
                {solve.speed_cs &&
                    <TableRow>
                        <TableCell>Time</TableCell>
                        <TableCell>{html_render_time(solve.speed_cs)}</TableCell>
                    </TableRow>
                }
                {solve.move_count &&
                    <TableRow>
                        <TableCell>Move count</TableCell>
                        <TableCell>{solve.move_count}</TableCell>
                    </TableRow>
                }
                <TableRow>
                    <TableCell>Solve date</TableCell>
                    <TableCell>{html_render_date(solve.solve_date)}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>Upload date</TableCell>
                    <TableCell>{html_render_date(solve.upload_date)}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>Program</TableCell>
                    <TableCell className="inline-flex items-center"><ProgramIcon abbr={solve.program.abbr}/>&nbsp;{solve.program.name}</TableCell>
                </TableRow>
                {solve.video_url &&
                    <TableRow>
                        <TableCell>Video link</TableCell>
                        <TableCell><a className="text-sidebar-primary hover:underline" href={solve.video_url}>{solve.video_url}</a></TableCell>
                    </TableRow>
                }
                {solve.log_file_name &&
                    <TableRow>
                        <TableCell>Log file</TableCell>
                        <TableCell>
                            <Attachment size={"xs"}>
                                <AttachmentMedia>
                                    <FileTextIcon/>
                                </AttachmentMedia>
                                <AttachmentContent>
                                    <AttachmentTitle>{solve.log_file_name}</AttachmentTitle>
                                </AttachmentContent>
                                <AttachmentActions>
                                    <a href={`/solve-file?id=${solve.id}`} download={`${solve.log_file_name}`}>
                                        <Button
                                        type="button"
                                        variant="default"
                                        size="icon"
                                        >
                                            <ArrowDownToLine className="size-4" />
                                        </Button>
                                    </a>
                                </AttachmentActions>
                            </Attachment>
                        </TableCell>
                    </TableRow>
                }

            </TableBody>
        </Table>
    )
}

export default SolveTable