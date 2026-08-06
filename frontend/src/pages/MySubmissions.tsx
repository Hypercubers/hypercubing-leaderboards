import Header from "@/components/header"
import SkeletonTableRows from "@/components/skeleton-table-rows";
import { TableHeader, TableRow, TableHead, Table, TableBody, TableCell } from "@/components/ui/table";
import { getUserSubmissions, type FullSolve } from "@/lib/backend"
import { html_render_date, html_render_time } from "@/lib/utils";
import { useEffect, useState } from "react"
import { Link, useNavigate, useSearchParams } from "react-router-dom";


function MySubmissions() {

    const [submissions, setSubmissions] = useState<FullSolve[]>([])

    let [searchParams] = useSearchParams();

    const searchQuery = searchParams.get("id")

    const navigate = useNavigate()

    useEffect(() => {
        getUserSubmissions(Number(searchQuery)).then(setSubmissions)
    }, [searchQuery])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">My Submissions</h1>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Puzzle</TableHead>
                        <TableHead>Time</TableHead>
                        <TableHead>Move count</TableHead>
                        <TableHead>Date</TableHead>
                        <TableHead>Program</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {submissions && submissions.length > 0 ? submissions.map((s) => (
                        <TableRow onClick={() => navigate(`/solve?id=${s.id}`)}>
                            <TableCell>{s.puzzle.name}</TableCell>
                            <TableCell>{s.speed_cs? html_render_time(s.speed_cs) : ""}</TableCell>
                            <TableCell>{s.move_count? s.move_count : ""}</TableCell>
                            <TableCell>{html_render_date(s.solve_date)}</TableCell>
                            <TableCell>{s.program.abbr}</TableCell>
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

export default MySubmissions
