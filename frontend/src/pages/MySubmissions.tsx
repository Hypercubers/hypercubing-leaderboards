import Header from "@/components/header"
import MySubmissionsTable from "@/components/table/my-submissions-table";
import { useAuth } from "@/lib/auth-context";
import { getUserSubmissions, type FullSolve } from "@/lib/api/solves"
import { useEffect, useState } from "react"


function MySubmissions() {

    const {user} = useAuth()

    const [submissions, setSubmissions] = useState<FullSolve[]|null>([])

    useEffect(() => {
        getUserSubmissions(user?.id ?? null).then(setSubmissions)
    }, [])

    return (
        <>
            <Header PageTitle="My submissions"/>
            <MySubmissionsTable FullSolves={submissions ?? []}/>
        </>
    )
}

export default MySubmissions
