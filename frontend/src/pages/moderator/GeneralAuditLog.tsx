import Header from "@/components/header"
import RenderedAuditLogEntryTable from "@/components/table/rendered-audit-log-entry-table"
import { getGeneralAuditLog, type RenderedAuditLogEntry } from "@/lib/backend"
import { useEffect, useState } from "react"



function GeneralAuditLog() {
    const [logs, setLogs] = useState<RenderedAuditLogEntry[] | null>(null)

    useEffect(() => {
        getGeneralAuditLog().then(setLogs)
    }, [])

    return (
        <>
        <Header PageTitle="General Audit Log"/>
        <p>Each solve and user page contains its own audit log. This page only lists other changes. </p>
        {logs && <RenderedAuditLogEntryTable logs={logs}/> }
        </>
    )
}

export default GeneralAuditLog