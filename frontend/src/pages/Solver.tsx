import Header from "@/components/header";
import SkeletonTableRows from "@/components/skeleton-table-rows";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { getUserPbs, type MainPageCategory, type PB, type RankedFullSolve } from "@/lib/backend";
import { html_render_date, html_render_time } from "@/lib/utils";
import { useEffect, useState } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";


interface params {
    id: number;
}

function Solver() {
    const navigate = useNavigate()

    const [searchParams, setSearchParams] = useSearchParams()
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solves, setSolves] = useState<[PB]>()

    useEffect(() => {
        getUserPbs(query.id).then(setSolves)
    }, [])

    return (
        <>
            <Header/>
            {/* get the user name in a better way */}
            <h1 className="text-4xl m-2">{solves && solves.length>0? solves[0][1].solve.solver.name : "b"}</h1>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Rank</TableHead>
                        <TableHead>Time</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {solves && solves.length > 0 ? solves.map((solve) => (
                        <TableRow onClick={() => navigate(`/solve?id=${solve[1].solve.id}`)}>
                            <TableCell>
                                <Link to={`/puzzle?id=${solve[1].solve.puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                    {solve[1].solve.puzzle.name}
                                </Link>
                            </TableCell>
                            <TableCell>{solve[1].rank}</TableCell>
                            <TableCell>{solve[1].solve.speed_cs && html_render_time(solve[1].solve.speed_cs)}</TableCell>
                            <TableCell>{html_render_date(solve[1].solve.solve_date)}</TableCell>
                            <TableCell>{solve[1].solve.program.abbr}</TableCell>
                        </TableRow>


                    ))
                    :
                    <SkeletonTableRows rows={5} cols={5}/>
                    }
                </TableBody>
            </Table>
        </>
    )

}

export default Solver;
