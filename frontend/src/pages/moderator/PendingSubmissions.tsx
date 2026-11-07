import Header from "@/components/header"
import PendingSubmissionsTable from "@/components/table/pending-submissions-table"
import { Card, CardHeader, CardTitle } from "@/components/ui/card"
import { getPendingSubmissions, type FullSolve } from "@/lib/backend"
import { useEffect, useState } from "react"


function PendingSubmissions() {

    const [submissions, setSubmissions] = useState<FullSolve[]>([])

    useEffect(() => {
        getPendingSubmissions().then(setSubmissions)
    }, [])

    
    return (
        <>
            <Header PageTitle="Pending Submissions"/>

            <Card className="m-2">
                <CardHeader>
                    <CardTitle>Verification queue is empty</CardTitle>
                </CardHeader>
            </Card>

            <PendingSubmissionsTable submissions={submissions} />
        </>
    )
}

export default PendingSubmissions