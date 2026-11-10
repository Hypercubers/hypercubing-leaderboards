import Header from "@/components/header"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getUser, getUserAuditLog, type PublicUser, type RenderedAuditLogEntry } from "@/lib/backend"
import { html_render_datetime } from "@/lib/utils"
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
export default UserAuditLog