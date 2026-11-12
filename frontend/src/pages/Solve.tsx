import Header from "@/components/header"
import RenderedAuditLogEntryTable from "@/components/table/rendered-audit-log-entry-table";
import SolveTable from "@/components/table/solve-table";
import { Button } from "@/components/ui/button";
import { Card, CardAction, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useAuth } from "@/lib/auth-context";
import { getSolve, getSolveLogs, verifySolve, type FullSolve, type RenderedAuditLogEntry } from "@/lib/backend"
import { get_youtube_id, html_render_time } from "@/lib/utils";
import { Check, Gavel, Pencil, Timer, X } from "lucide-react";
import { useState, useEffect } from "react"
import { Link, useSearchParams } from "react-router-dom"
import Error from "./Error";
import { ButtonGroup, ButtonGroupText } from "@/components/ui/button-group";
import { Input } from "@/components/ui/input";

interface SolveIDParams {
    id: number;
}

function Solve() {

    const { user } = useAuth()

    const [verificationComment, setVerificationComment] = useState<string>("")
    

    const [searchParams] = useSearchParams()
    const query: SolveIDParams = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solve, setSolve] = useState<FullSolve>()

    // const [canEdit, setCanEdit] = useState(false)

    const [logs, setLogs] = useState<RenderedAuditLogEntry[] | null>(null)

    const [solveNotFound, setSolveNotFound] = useState(false)

    useEffect(() => {
        getSolveLogs(query.id).then(setLogs)
    }, [])

    useEffect(() => {
    void getSolve(query.id).then((result) => {
        if (result) {
            setSolve(result)
        } else {
            setSolveNotFound(true)
        }
        })
    }, [query.id])

    const title = solve? `${solve.puzzle.name} ${solve.speed_cs? ` in ${html_render_time(solve.speed_cs)}`:""} ${solve.move_count? ` and ${solve.move_count} STM `:""} by ${solve.solver.name}` : `Solve ${query.id}`

    const canEdit = user !== undefined && user !== null && (user.moderator || (solve && user.id === solve.solver.id))

    if (solveNotFound) {
        return <Error code="404" msg="Solve not found." />
    }

    return (
        <>
        <Header PageTitle={title}/>

        {solve && 
        <>
            {user?.moderator && 
            <Card className="m-2">
                <CardHeader>
                    <CardTitle>Solve verification</CardTitle>
                    <CardAction>
                        <Link to={`/audit-log/solve?id=${solve.id}`}><Button><Gavel/> View solve audit log</Button></Link>
                    </CardAction>
                </CardHeader>
                <CardContent className="*:mb-2">
                    {solve.speed_cs && 
                        <ButtonGroup>
                            <ButtonGroupText>Speed</ButtonGroupText>
                            <Button onClick={() => verifySolve(solve.id, "speed", "accept", verificationComment)} disabled={solve.speed_verified === true}><Check/> Accept</Button>
                            <Button onClick={() => verifySolve(solve.id, "speed", "reject", verificationComment)} disabled={solve.speed_verified === false}><X/> Reject</Button>
                            <Button onClick={() => verifySolve(solve.id, "speed", "unverify", verificationComment)} disabled={solve.speed_verified === null || solve.speed_verified === undefined}><Timer/> Unverify</Button>
                        </ButtonGroup>
                    }
                    {solve.move_count &&
                        <ButtonGroup>
                            <ButtonGroupText>Fewest-moves</ButtonGroupText>
                            <Button onClick={() => verifySolve(solve.id, "fmc", "accept", verificationComment)} disabled={solve.fmc_verified === true}><Check/> Accept</Button>
                            <Button onClick={() => verifySolve(solve.id, "fmc", "reject", verificationComment)} disabled={solve.fmc_verified === false}><X/> Reject</Button>
                            <Button onClick={() => verifySolve(solve.id, "fmc", "unverify", verificationComment)} disabled={solve.fmc_verified === null || solve.fmc_verified === undefined}><Timer/> Unverify</Button>
                        </ButtonGroup>
                    }
                    <Input placeholder="Comment with verification..." value={verificationComment} onChange={(e) => setVerificationComment(e.target.value)} />
                </CardContent>
            </Card>
            }
            <Card className="m-2">
                <CardHeader>
                    <CardTitle>Solve details</CardTitle>
                    <CardAction>
                        {canEdit && <Link to={`/edit-solve?id=${solve.id}`}><Button><Pencil/> Edit solve</Button></Link>}
                    </CardAction>
                </CardHeader>
                <CardContent>
                    {solve.video_url && <iframe className="w-full md:w-1/2 aspect-video" src={`https://www.youtube-nocookie.com/embed/${get_youtube_id(solve.video_url)}`} allow="encrypted-media"></iframe>}
                    <SolveTable solve={solve}/>
                </CardContent>
            </Card>
                
            <h1 className="text-4xl mt-2">History</h1>
            {logs && <RenderedAuditLogEntryTable logs={logs}/> }
        </>
        }
        </>
    )

}

export default Solve
