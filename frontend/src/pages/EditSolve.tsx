import Header from "@/components/header"
import SolveForm from "@/components/solve-form"
import { useAuth } from "@/lib/auth-context"
import { getSolve, submitSolve, type FullSolve } from "@/lib/backend"
import { useEffect, useState } from "react"
import { useNavigate, useSearchParams } from "react-router-dom"
import Error from "./Error"
import { solveToFormValues } from "@/lib/utils"

function EditSolve() {
    const { user } = useAuth()
    const navigate = useNavigate()
    const [searchParams] = useSearchParams()
    const solveId = Number(searchParams.get("id"))
    const [solve, setSolve] = useState<FullSolve | null>(null)

    useEffect(() => {
        if (Number.isInteger(solveId) && solveId > 0) {
            void getSolve(solveId).then(setSolve)
        }
    }, [solveId])

    const isLoading = user === undefined || solve === null
    const canEdit = solve !== null
        && user != null
        && (user.moderator
            || (user.id === solve.solver.id
                && solve.fmc_verified !== true
                && solve.speed_verified !== true))

    return (
        <>
            {isLoading && <p className="m-2">Loading solve...</p>}
            {!isLoading && !canEdit && (
                <Error code="403" msg="Forbidden. You are not authorized to edit this solve."/>
            )}
            {canEdit && solve && (
                <>
                <Header PageTitle="Edit Solve" /><SolveForm
                    initialValues={solveToFormValues(solve)}
                    submitLabel="Save changes"
                    onSubmit={async (value) => {
                        const response = await submitSolve(
                            {
                                ...value,
                                solve_id: solve.id,
                            },
                            "edit-solve"
                        )
                        if (response) {
                            navigate(response.redirect)
                        }
                    } } />
                </>
            )}
        </>
    )
}

export default EditSolve
