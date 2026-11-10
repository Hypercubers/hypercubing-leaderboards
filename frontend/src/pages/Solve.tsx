import Header from "@/components/header"
import SkeletonSolveCard from "@/components/skeleton-solve-card";
import SolveTable from "@/components/table/solve-table";
import { Button } from "@/components/ui/button";
import { Card, CardAction, CardContent, CardHeader } from "@/components/ui/card";
import { useAuth } from "@/lib/auth-context";
import { getSolve, type FullSolve } from "@/lib/backend"
import { get_youtube_id, html_render_time } from "@/lib/utils";
import { Pencil } from "lucide-react";
import { useState, useEffect } from "react"
import { useNavigate, useSearchParams } from "react-router-dom"

interface SolveIDParams {
    id: number;
}

function Solve() {

    const navigate = useNavigate()

    const { user } = useAuth()
    

    const [searchParams] = useSearchParams()
    const query: SolveIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solve, setSolve] = useState<FullSolve>()

    const [canEdit, setCanEdit] = useState(false)

    const [title, setTitle] = useState<string>("")

    useEffect(() => {
        getSolve(query.id).then(setSolve)
    }, [])

    useEffect(() => {
        if (solve) {
            setTitle(`${solve.puzzle.name} ${solve.speed_cs? ` in ${html_render_time(solve.speed_cs)}`:""} ${solve.move_count? ` and ${solve.move_count} STM `:""} by ${solve.solver.name}`)
        }
    }, [solve])

    useEffect(() => {
        if (user && solve) {
            setCanEdit(
                user !== undefined &&
                user !== null &&
                (user.moderator || user.id === solve.solver.id)
            )
        }
    }, [user, solve])

    return (
        <>
        <Header PageTitle={title}/>
        {solve?
        <Card className="m-2">
            <CardHeader>
                <CardAction>
                    {canEdit && <Button onClick={() => navigate(`/edit-solve?id=${solve.id}`)}><Pencil/> Edit</Button>}
                </CardAction>
            </CardHeader>
            <CardContent>
                {solve.video_url && <iframe className="w-full md:w-1/2 aspect-video" src={`https://www.youtube-nocookie.com/embed/${get_youtube_id(solve.video_url)}`} allow="encrypted-media"></iframe>}
                <SolveTable solve={solve}/>
            </CardContent>
        </Card>
            : <SkeletonSolveCard/>
            }
            <h1 className="text-4xl mt-2">History</h1>
        </>
    )

}

export default Solve
