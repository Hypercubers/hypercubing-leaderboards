import Header from "@/components/header"
import { getGeneralAuditLog, type RenderedAuditLogEntry } from "@/lib/backend"
import { useEffect, useState } from "react"


function AuditLog() {
    const [logs, setLogs] = useState<RenderedAuditLogEntry[] | null>(null)

    useEffect(() => {
        getGeneralAuditLog().then(setLogs)
    }, [])

    return (
        <>
        <Header PageTitle="General Audit Log"/>
        <p>Each solve and user page contains its own audit log. This page only lists other changes. </p>
        {logs && (
            <ul>
                {logs.map((log, index) => (
                    <li key={index}>
                        <strong>{log.timestamp}</strong> - {log.editor_name} ({log.editor_url}): {log.description}
                    </li>
                ))}
            </ul>
        )}
        </>
    )
}

export default AuditLog