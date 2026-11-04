import Header from "@/components/header"
import { Table, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getGeneralAuditLog, type RenderedAuditLogEntry } from "@/lib/backend"
import { html_render_date } from "@/lib/utils"
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
        {logs && (
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Timestamp</TableHead>
                        <TableHead>Editor</TableHead>
                        <TableHead>Description</TableHead>
                    </TableRow>
                </TableHeader>
                {logs.map((log, index) => (
                    <TableRow key={index}>
                        <TableCell>{html_render_date(log.timestamp)}</TableCell>
                        <TableCell><a className="text-sidebar-primary hover:underline" href={log.editor_url}>{log.editor_name}</a></TableCell>
                        <TableCell className="whitespace-pre">{log.description}</TableCell>
                    </TableRow>
                ))}
            </Table>
        )}
        </>
    )
}

export default GeneralAuditLog