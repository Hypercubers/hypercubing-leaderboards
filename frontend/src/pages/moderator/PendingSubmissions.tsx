import Header from "@/components/header"
import PendingSubmissionsTable from "@/components/table/pending-submissions-table"
import { Card, CardHeader, CardTitle } from "@/components/ui/card"


function PendingSubmissions() {
    return (
        <>
            <Header PageTitle="Pending Submissions"/>

            <Card className="m-2">
                <CardHeader>
                    <CardTitle>Verification queue is empty</CardTitle>
                </CardHeader>
            </Card>

            <PendingSubmissionsTable/>
        </>
    )
}

export default PendingSubmissions