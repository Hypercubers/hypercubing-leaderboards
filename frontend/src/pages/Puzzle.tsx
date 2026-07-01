import Header from "@/components/header"
import SkeletonTableRows from "@/components/skeleton-table-rows";
import { Skeleton } from "@/components/ui/skeleton";
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { getPuzzleSolves, type FullSolve, type RankedFullSolve } from "@/lib/backend";
import { html_render_date, html_render_time } from "@/lib/utils";
import { useEffect, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom"

interface PuzzleIDParams {
    id: number;
}

function Puzzle() {
    const navigate = useNavigate()

    const [searchParams, setSearchParams] = useSearchParams()
    const query: PuzzleIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solves, setSolves] = useState<RankedFullSolve[]>([])

    useEffect(() => {
        getPuzzleSolves(query.id).then(setSolves)
        }, [])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">{solves && solves.length > 0 ? solves[0].solve.puzzle.name : "Unknown Puzzle"}</h1>

            <Table>
                <TableHeader>
                    <TableRow >
                        <TableHead>Rank</TableHead>
                        <TableHead>Solver</TableHead>
                        <TableHead>Time</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {solves && solves.length > 0 ? solves.map((s) => (
                        <TableRow onClick={() => navigate(`/solve?id=${s.solve.id}`)}>
                            <TableCell>{s.rank}</TableCell>
                            <TableCell>
                                <Link to={`/user?id=${s.solve.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                    {s.solve.solver.name}
                                </Link>
                            </TableCell>
                            <TableCell>{s.solve.speed_cs? html_render_time(s.solve.speed_cs) : "no time"}</TableCell>
                            <TableCell>{html_render_date(s.solve.solve_date)}</TableCell>
                            <TableCell>{s.solve.program.abbr}</TableCell>
                        </TableRow>
                    ))
                :
                <SkeletonTableRows rows={4} cols={5}/>
                }

                </TableBody>
            </Table>
        </>
    )
}

export default Puzzle
