import CategoryQuerySelector from "@/components/category-query-selector";
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { getPuzzleInfo, getPuzzleSolves, type RankedFullSolve, type Puzzle } from "@/lib/backend";
import { html_render_date, html_render_time, url_params_to_category_query } from "@/lib/utils";
import { useEffect, useMemo, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom"

function Puzzle() {
    const navigate = useNavigate()

    const [searchParams] = useSearchParams()
    const id = Number(searchParams.get("id")) || 1
    const [solves, setSolves] = useState<RankedFullSolve[]>([])
    const [puzzle, setPuzzle] = useState<Puzzle>()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    useEffect(() => {
        getPuzzleInfo(id).then(setPuzzle)
    }, [])

    useEffect(() => {
        getPuzzleSolves(id, categoryQuery).then(setSolves)
    }, [id, searchParams.toString()]) // .toString() so the effect keys off actual content

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">{puzzle && `${puzzle.name}`}</h1>

            <CategoryQuerySelector puzzleId={id} query={categoryQuery} />

            <Table>
                <TableHeader>
                    <TableRow >
                        <TableHead>Rank</TableHead>
                        <TableHead>Solver</TableHead>
                        <TableHead>{searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca" ? "Move count" : "Time"}</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                {solves &&
                <TableBody>
                    {solves.length > 0 ? solves.map((s) => (
                        <TableRow onClick={() => navigate(`/solve?id=${s.solve.id}`)}>
                            <TableCell>{s.rank}</TableCell>
                            <TableCell>
                                <Link to={`/solver?id=${s.solve.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                    {s.solve.solver.name}
                                </Link>
                            </TableCell>
                            <TableCell>
                                {searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca" ?
                                s.solve.move_count && s.solve.move_count :
                                s.solve.speed_cs && html_render_time(s.solve.speed_cs)}
                            </TableCell>
                            <TableCell>{html_render_date(s.solve.solve_date)}</TableCell>
                            <TableCell className="inline-flex items-center"><ProgramIcon abbr={s.solve.program.abbr}/>&nbsp;{s.solve.program.abbr}</TableCell>
                        </TableRow>
                    ))
                    :
                        <p className="pt-4">There are no solves matching the search.</p>
                    }
                </TableBody>
            }
            </Table>
        </>
    )
}

export default Puzzle
