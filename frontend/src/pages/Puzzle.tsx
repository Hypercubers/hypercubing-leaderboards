import CategoryQuerySelector from "@/components/category-query-selector";
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon";
import SkeletonTableRows from "@/components/skeleton-table-rows";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { getPuzzleSolves, type CategoryQuery, type RankedFullSolve } from "@/lib/backend";
import { html_render_date, html_render_time, puz_flags } from "@/lib/utils";
import { useEffect, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom"

interface PuzzleIDParams {
    id: number;
}

function Puzzle() {
    const navigate = useNavigate()

    const [searchParams] = useSearchParams()

    const query: PuzzleIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [categoryQuery, setCategoryQuery] = useState<CategoryQuery>({
            Speed: {
                average: false,
                blind: false,
                filters: undefined,
                macros: undefined,
                one_handed: false,
                variant: "Default",
                program: "Default",
            },
            Fmc: {
                enabled: false,
                computer_assisted: false,
            },
        })

        function handleQueryChange(value: CategoryQuery) {
            setCategoryQuery(value)
        }

    const [solves, setSolves] = useState<RankedFullSolve[]>([])

    useEffect(() => {
        getPuzzleSolves(query.id, categoryQuery).then(setSolves)
    }, [query.id, categoryQuery])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">{solves && solves.length > 0 ? `${solves[0].solve.puzzle.name} ${puz_flags(solves[0].solve.flags)}` : "Unknown Puzzle"}</h1>

            <CategoryQuerySelector puzzleId={query.id} onSendQuery={handleQueryChange}/>

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
                                <Link to={`/solver?id=${s.solve.solver.id}`} onClick={(e) => e.stopPropagation()}>
                                    {s.solve.solver.name}
                                </Link>
                            </TableCell>
                            <TableCell>{s.solve.speed_cs? html_render_time(s.solve.speed_cs) : "no time"}</TableCell>
                            <TableCell>{html_render_date(s.solve.solve_date)}</TableCell>
                            <TableCell className="inline-flex items-center"><ProgramIcon abbr={s.solve.program.abbr}/>&nbsp;{s.solve.program.abbr}</TableCell>
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
