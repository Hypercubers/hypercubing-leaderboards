import Header from "@/components/header"
import RenderedAuditLogEntryTable from "@/components/table/rendered-audit-log-entry-table"
import {getUserAuditLog, type RenderedAuditLogEntry } from "@/lib/api/audit"
import { getUser, type PublicUser } from "@/lib/api/users"
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
        getUserAuditLog(query.id).then(setLogs)
    }, [])

    return (
        <>
            <Header PageTitle={`Audit Log for ${solverName?.name ?? `User ${query.id}`}`}/>
            {logs && <RenderedAuditLogEntryTable logs={logs}/> }
        </>
    )
}
export default UserAuditLog