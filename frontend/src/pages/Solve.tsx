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

    useEffect(() => {
            getSolve(query.id).then(setSolve)
          }, [])

    return (
        <>
        <Header/>
        <h1 className="text-4xl m-2">Solve</h1>
        {solve?
        <Card>
            <CardHeader>
                <CardTitle>
                    {`${solve.puzzle.name} in ${solve.speed_cs} by ${solve.solver.name}`}
                </CardTitle>
            </CardHeader>
            <CardContent>
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
