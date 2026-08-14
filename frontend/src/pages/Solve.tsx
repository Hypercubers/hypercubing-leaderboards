import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon";
import SkeletonSolveCard from "@/components/skeleton-solve-card";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { getSolve, type FullSolve } from "@/lib/backend"
import { get_youtube_id, html_render_date, html_render_time } from "@/lib/utils";
import { useState, useEffect } from "react"
import { Link, useSearchParams } from "react-router-dom"

interface SolveIDParams {
    id: number;
}

function Solve() {

    const [searchParams, setSearchParams] = useSearchParams()
    const query: SolveIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solve, setSolve] = useState<FullSolve>()

    useEffect(() => {
        getSolve(query.id).then(setSolve)
    }, [])

    return (
        <>
        <Header/>
        <h1 className="text-4xl m-2">{solve? `Solve #${solve.id}` : "Unknown"}</h1>
        {solve?
        <Card>
            <CardHeader>
                <CardTitle>
                    {`${solve.puzzle.name} ${solve.speed_cs? ` in ${html_render_time(solve.speed_cs)}`:""} ${solve.move_count? ` and ${solve.move_count}`:""} by ${solve.solver.name}`}
                </CardTitle>
            </CardHeader>
            <CardContent>
                {solve.video_url && <iframe width="560" height="315" src={`https://www.youtube-nocookie.com/embed/${get_youtube_id(solve.video_url)}`} allow="encrypted-media"></iframe>}
                <Table>
                    <TableBody>
                        <TableRow>
                            <TableCell>Solver</TableCell>
                            <TableCell>
                                <Link to={`/solver?id=${solve.solver.id}`}>
                                    {solve.solver.name}
                                </Link>
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Puzzle</TableCell>
                            <TableCell>
                                <Link to={`/puzzle?id=${solve.puzzle.id}`}>
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
                                <TableCell><a href={solve.video_url}>{solve.video_url}</a></TableCell>
                            </TableRow>
                        }
                        {solve.log_file_name &&
                            <TableRow>
                                <TableCell>Log file</TableCell>
                                <TableCell>{solve.log_file_name}</TableCell>
                            </TableRow>
                        }

                    </TableBody>
                </Table>
            </CardContent>
        </Card>
            : <SkeletonSolveCard/>
            }
        </>
    )

}

export default Solve
