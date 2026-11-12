import Header from "@/components/header"
import PendingSubmissionsTable from "@/components/table/pending-submissions-table"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { autoVerification, getPendingSubmissions, type FullSolve } from "@/lib/backend"
import { useEffect, useState } from "react"


function PendingSubmissions() {

    const [submissions, setSubmissions] = useState<FullSolve[]>([])
    const [autoVerifying, setAutoVerifying] = useState(false)
    const [autoVerificationDescription, setAutoVerificationDescription] = useState(
        "Some solves may still need manual review."
    )

    useEffect(() => {
        getPendingSubmissions().then(setSubmissions)
    }, [])

    async function runAutoVerification() {
        setAutoVerifying(true)
        setAutoVerificationDescription("Auto verification started...")

        try {
            const started = await autoVerification()
            if (!started) {
                setAutoVerificationDescription("Unable to start auto verification.")
                return
            }

            for (let refresh = 0; refresh < 2; refresh++) {
                await new Promise(resolve => setTimeout(resolve, 1000))
                const refreshedSubmissions = await getPendingSubmissions()
                if (refreshedSubmissions) {
                    setSubmissions(refreshedSubmissions)
                }
            }

            setAutoVerificationDescription("Auto verification results have been refreshed.")
        } catch {
            setAutoVerificationDescription("Unable to complete auto verification.")
        } finally {
            setAutoVerifying(false)
        }
    }

    
    return (
        <>
            <Header PageTitle="Pending Submissions"/>

            <Card className="m-2">
                <CardHeader>
                    <CardTitle>Auto verification</CardTitle>
                    <CardDescription>{autoVerificationDescription}</CardDescription>
                </CardHeader>
                <CardContent>
                    <Button onClick={() => { void runAutoVerification() }} disabled={autoVerifying}>
                        {autoVerifying ? "Running auto verification..." : "Run auto verification on ALL pending solves"}
                    </Button>
                </CardContent>
            </Card>

            <PendingSubmissionsTable submissions={submissions} />
        </>
    )
}

export default PendingSubmissions