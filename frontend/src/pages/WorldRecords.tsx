import CategoryQuerySelector from "@/components/category-query-selector"
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getWorldRecords, type Record, type CategoryQuery } from "@/lib/backend"
import { html_render_date, html_render_time, puz_name } from "@/lib/utils"
import { useEffect, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom"


function WorldRecords() {
    const navigate = useNavigate()
    const [searchParams, setSearchParams] = useSearchParams();

    const [records, setRecords] = useState<Record[]>([])

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

    useEffect(() => {
        getWorldRecords(categoryQuery).then(setRecords)
    }, [categoryQuery])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">World Records</h1>

            <CategoryQuerySelector onSendQuery={handleQueryChange}/>

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
        </>
    )

}

export default WorldRecords
