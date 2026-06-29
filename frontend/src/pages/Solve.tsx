import Header from "@/components/header"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { getSolve, type FullSolve } from "@/lib/backend"
import { get_youtube_id, html_render_date, html_render_time } from "@/lib/utils";
import { useState, useEffect } from "react"
import { useSearchParams } from "react-router-dom"

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

            console.log(solve?.video_url)
            console.log(solve?.video_url? get_youtube_id(solve.video_url) : "no video url")
          }, [])

    return (
        <>
        <Header/>
        <h1 className="text-4xl m-2">Solve{solve? ` #${solve.id}` : ""}</h1>
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
                            <TableCell>{solve.solver.name}</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>Puzzle</TableCell>
                            <TableCell>{solve.puzzle.name}</TableCell>
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
                            <TableCell>{solve.program.name}</TableCell>
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
            : <p>Invalid solve ID</p>}
        </>
    )

}

export default Solve
