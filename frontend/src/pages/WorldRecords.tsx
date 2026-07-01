import Header from "@/components/header"
import SkeletonTableRows from "@/components/skeleton-table-rows"
import { Button } from "@/components/ui/button"
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getPuzzles, getWorldRecords, type Puzzle, type Record } from "@/lib/backend"
import { html_render_date, html_render_time } from "@/lib/utils"
import { useEffect, useState } from "react"
import { Link, NavLink, useNavigate } from "react-router-dom"


function WorldRecords() {
    const navigate = useNavigate()

    const [records, setRecords] = useState<Record[]>([])

      useEffect(() => {
        // getPuzzles().then(setPuzzles)
        getWorldRecords().then(setRecords)
      }, [])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">World Records</h1>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Record Holder</TableHead>
                        <TableHead>Time</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {records && records.length > 0 ? records.map((rec) => (
                        <TableRow onClick={() =>  navigate(`/solve?id=${rec[1].id}`)}>
                            <TableCell>
                                <Link to={`/puzzle?id=${rec[1].puzzle.id}`} onClick={(e) => e.stopPropagation()}>
                                    {rec[0].puzzle.name}
                                </Link>
                            </TableCell>

                            <TableCell>
                                <Link to={`/user?id=${rec[1].solver.id}`} onClick={(e) => e.stopPropagation()}>
                                    {rec[1].solver.name}
                                </Link>
                            </TableCell>

                            <TableCell>
                                {rec[1].speed_cs? html_render_time(rec[1].speed_cs) : "no time"}
                            </TableCell>

                            {/* <TableCell>{rec[1].speed_cs? html_render_time(rec[1].speed_cs) : "no time"}</TableCell> */}
                            <TableCell>{html_render_date(rec[1].solve_date)}</TableCell>
                            <TableCell>{rec[1].program.abbr}</TableCell>

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

export default WorldRecords
