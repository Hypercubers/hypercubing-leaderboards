import CategoryQuerySelector from "@/components/category-query-selector"
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon"
import DistinctTable from "@/components/table/distinct-table"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getDistinctRecords, getWorldRecords, type Distinct, type Record } from "@/lib/backend"
import { html_render_date, html_render_time, puz_name, url_params_to_category_query } from "@/lib/utils"
import { useEffect, useMemo, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom"


function WorldRecords() {
    const navigate = useNavigate()
    const [searchParams] = useSearchParams();

    const [records, setRecords] = useState<Record[]>([])
    const [distinct, setDistinct] = useState<Distinct[]>([])

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    const isDistinct = categoryQuery.distinct

    useEffect(() => {
        if (isDistinct) {
            getDistinctRecords().then(setDistinct)
        } else {
            getWorldRecords(categoryQuery).then(setRecords)
        }
    }, [categoryQuery, isDistinct])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">World Records</h1>

            <CategoryQuerySelector query={categoryQuery} showAggregates/>

            {categoryQuery.distinct ?
                <DistinctTable Records={distinct}/>
            :


            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Record Holder</TableHead>
                        <TableHead>{searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca" ? "Move count" : "Time"}</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {records && records.length > 0 ? records.map((rec) => (
                        <TableRow onClick={() =>  navigate(`/solve?id=${rec[1].id}`)}>
                            <TableCell>
                                <Link to={`/puzzle?id=${rec[1].puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                    {`${puz_name(rec[0])}`}
                                </Link>
                            </TableCell>

                            <TableCell>
                                <Link to={`/solver?id=${rec[1].solver.id}`} onClick={(e) => e.stopPropagation()}>
                                    {rec[1].solver.name}
                                </Link>
                            </TableCell>

                            <TableCell>
                                {searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca" ?
                                rec[1].move_count && rec[1].move_count :
                                rec[1].speed_cs && html_render_time(rec[1].speed_cs)}
                            </TableCell>

                            {/* <TableCell>{rec[1].speed_cs? html_render_time(rec[1].speed_cs) : "no time"}</TableCell> */}
                            <TableCell>{html_render_date(rec[1].solve_date)}</TableCell>
                            <TableCell className="inline-flex items-center"><ProgramIcon abbr={rec[1].program.abbr}/>&nbsp;{rec[1].program.abbr}</TableCell>

                        </TableRow>
                    ))
                    :
                    <></>
                    }
                </TableBody>
            </Table>
            }
        </>
    )

}

export default WorldRecords
