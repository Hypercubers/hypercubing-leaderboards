import Header from "@/components/header"
import MySubmissionsTable from "@/components/table/my-submissions-table"
import { getUserSubmissions, type FullSolve } from "@/lib/backend"
import { useEffect, useState } from "react"
import { useSearchParams } from "react-router-dom"


interface SolverIDParams {
    id: number;
}

function SolverSubmissions() {

    const [submissions, setSubmissions] = useState<FullSolve[]>([])

    const [searchParams] = useSearchParams()
    const query: SolverIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    useEffect(() => {
        getUserSubmissions(query.id).then(setSubmissions)
    }, [])
    
    return (
        <>
            <Header PageTitle={`Submissions from ${submissions[0]?.solver?.name || `Solver ${query.id}`}`} />
            <MySubmissionsTable FullSolves={submissions}/>
        </>
    )
}

export default SolverSubmissions