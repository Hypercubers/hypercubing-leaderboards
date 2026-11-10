import Header from "@/components/header";
import RenderedAuditLogEntryTable from "@/components/table/rendered-audit-log-entry-table";
import { getSolveAuditLog, type RenderedAuditLogEntry } from "@/lib/backend";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";



interface params {
    id: number;
}

function SolveAuditLog() {
    const [searchParams] = useSearchParams()
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    const [logs, setLogs] = useState<RenderedAuditLogEntry[] | null>(null)

    useEffect(() => {
            getSolveAuditLog(query.id).then(setLogs)
        }, [])

    return (
        <>
        <Header PageTitle={`Audit log for solve #${query.id}`}/>

        {logs && <RenderedAuditLogEntryTable logs={logs}/> }
        </>
    )
}

export default SolveAuditLog