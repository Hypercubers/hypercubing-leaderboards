import CategoryQuerySelector from "@/components/category-query-selector";
import Header from "@/components/header";
import PbTable from "@/components/pb-table";
import { getUser, getUserPbs, type PublicUser, type PB } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";


interface params {
    id: number;
}

function Solver() {
    const [searchParams] = useSearchParams()
    const [solverName, setSolverName] = useState<PublicUser|null>(null)
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solves, setSolves] = useState<[PB]>()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    useEffect(() => {
        getUser(query.id).then(setSolverName)
    }, [])

    useEffect(() => {
        getUserPbs(query.id, categoryQuery).then(setSolves)
    }, [categoryQuery])

    return (
        <>
            <Header/>
            {/* get the user name in a better way */}
            <h1 className="text-4xl m-2">{solverName?.name ?? `Solver ${query.id}`}</h1>

            <CategoryQuerySelector query={categoryQuery}/>

            <PbTable PBs={solves} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>

            {/* <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Rank</TableHead>
                        <TableHead>{searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca" ? "Move count" : "Time"}</TableHead>
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
                            <TableCell className="inline-flex items-center"><ProgramIcon abbr={solve[1].solve.program.abbr}/>&nbsp;{solve[1].solve.program.abbr}</TableCell>
                        </TableRow>


                    ))
                    :
                    <SkeletonTableRows rows={5} cols={5}/>
                    }
                </TableBody>
            </Table> */}
        </>
    )

}

export default Solver;
