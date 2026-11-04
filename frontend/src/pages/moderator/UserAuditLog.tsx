import Header from "@/components/header"
import { Table, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getGeneralAuditLog, getUser, type PublicUser, type RenderedAuditLogEntry } from "@/lib/backend"
import { html_render_date } from "@/lib/utils"
import { useEffect, useState } from "react"
import { useSearchParams } from "react-router-dom"

interface params {
    id: number;
}

function UserAuditLog() {
    const [searchParams] = useSearchParams()
    const [solverName, setSolverName] = useState<PublicUser|null>(null)
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    useEffect(() => {
        getUser(query.id).then(setSolverName)
    }, [])




    const [logs, setLogs] = useState<RenderedAuditLogEntry[] | null>(null)

    useEffect(() => {
        getGeneralAuditLog().then(setLogs)
    }, [])

    return (
        <>
            <Header PageTitle={`Audit Log for ${solverName?.name ?? `User ${query.id}`}`}/>
        </>
    )
}

export default UserAuditLog