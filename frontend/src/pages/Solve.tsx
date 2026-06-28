import Header from "@/components/header"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { getSolve, type FullSolve } from "@/lib/backend"
import { useState, useEffect } from "react"
import { useSearchParams } from "react-router-dom"

interface SolveIDParams {
    id: number;
}

function Solve() {

    const [searchParams, setSearchParams] = useSearchParams()
    const query: SolveIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solve, setSolve] = useState<FullSolve>()
    const [videoID, setVideoID] = useState("")

    useEffect(() => {
            getSolve(query.id).then(setSolve)
            // .then(setVideoID(() =>: string {
            //     return 'a'
            // }))
          }, [])

    return (
        <>
        <Header/>
        <h1 className="text-4xl m-2">Solve{solve? ` #${solve.id}` : ""}</h1>
        {solve?
        <Card>
            <CardHeader>
                <CardTitle>
                    {`${solve.puzzle.name} in ${solve.speed_cs} by ${solve.solver.name}`}
                </CardTitle>
            </CardHeader>
            <CardContent>
                <p>{solve.video_url}</p>
                <iframe width="560" height="315" src={`https://www.youtube-nocookie.com/embed/${solve.video_url}`} allow="encrypted-media"></iframe>
                <p>{solve.puzzle.name}</p>
                <p>{solve.program.name}</p>
                <p>{solve.solve_date}</p>
                <p>{solve.speed_cs}</p>
                <p>{solve.solver.name}</p>
            </CardContent>


        </Card>
            : <p>Invalid solve ID</p>}
        </>
    )

}

export default Solve
