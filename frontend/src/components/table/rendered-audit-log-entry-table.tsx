import { html_render_datetime } from "@/lib/utils"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table"
import type { RenderedAuditLogEntry } from "@/lib/backend"


function RenderedAuditLogEntryTable({ logs }: { logs: RenderedAuditLogEntry[] }) {
    return (
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
    )
}

export default RenderedAuditLogEntryTable