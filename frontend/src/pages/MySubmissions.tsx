import Header from "@/components/header"
import MySubmissionsTable from "@/components/table/my-submissions-table";
import { useAuth } from "@/lib/auth-context";
import { getUserSubmissions, type FullSolve } from "@/lib/backend"
import { useEffect, useState } from "react"


function MySubmissions() {

    const {user} = useAuth()

    const [submissions, setSubmissions] = useState<FullSolve[]>([])

    useEffect(() => {
        getUserSubmissions(user?.id).then(setSubmissions)
    }, [])

    return (
        <>
            <Header PageTitle="My submissions"/>
            <MySubmissionsTable FullSolves={submissions}/>
        </>
    )
}

export default MySubmissions
