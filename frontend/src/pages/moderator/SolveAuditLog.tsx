import Header from "@/components/header";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { getSolveAuditLog, type RenderedAuditLogEntry } from "@/lib/backend";
import { html_render_datetime } from "@/lib/utils";
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

        {logs && (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>Timestamp</TableHead>
                    <TableHead>Editor</TableHead>
                    <TableHead>Description</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
            {logs.map((log, index) => (
                <TableRow key={index}>
                    <TableCell>{html_render_datetime(log.timestamp)}</TableCell>
                    <TableCell><a className="text-sidebar-primary hover:underline" href={log.editor_url}>{log.editor_name}</a></TableCell>
                    <TableCell className="whitespace-pre">{log.description}</TableCell>
                </TableRow>
            ))}
            </TableBody>
        </Table>
        )}
        </>
    )
}

export default SolveAuditLog