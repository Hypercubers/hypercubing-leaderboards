import Header from "@/components/header"
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getPuzzles, getWorldRecords, type Puzzle, type Record } from "@/lib/backend"
import { html_render_date, html_render_time } from "@/lib/utils"
import { useEffect, useState } from "react"


function WorldRecords() {

    const [puzzles, setPuzzles] = useState<Puzzle[]>([])

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
                <TableCaption>A list of world records</TableCaption>
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
                    {records.length > 0 ? records.map((rec) => (
                        <TableRow>
                            <TableCell>{rec[0].puzzle.name}</TableCell>
                            <TableCell>{rec[1].solver.name}</TableCell>
                            <TableCell>{rec[1].speed_cs? html_render_time(rec[1].speed_cs) : "no time"}</TableCell>
                            <TableCell>{html_render_date(rec[1].solve_date)}</TableCell>
                            <TableCell>{rec[1].program.abbr}</TableCell>
                        </TableRow>
                    ))
                    :
                    <TableRow>
                        <TableCell>Nothing here</TableCell>
                    </TableRow>
                    }
                </TableBody>
            </Table>
        </>
    )

}

export default WorldRecords
