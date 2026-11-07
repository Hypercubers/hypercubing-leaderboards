import Header from "@/components/header"
import SolveForm, { createEmptySolveData } from "@/components/solve-form"
import { submitSolve } from "@/lib/backend"
import { useAuth } from "@/lib/auth-context"
import { useNavigate } from "react-router-dom"

function SubmitSolve() {
    const { user } = useAuth()
    const navigate = useNavigate()

    return (
        <>
            <Header PageTitle="Submit Solve"/>

            <div className="m-2">
                <p>Read the <a className="text-sidebar-primary hover:underline" href="https://hypercubing.xyz/leaderboards/rules/">Hypercubing Leaderboard Submission Rules</a> before submitting. Additionally, note that: </p>
                <ul className="list-disc pt-4 pl-6">
                    <li>Average of 5 submissions are only accepted for speedsolves of 3<sup>4</sup> (virtual), 2<sup>4</sup> (virtual and physical), and 2<sup>5</sup> (virtual)</li>
                    <li>3<sup>3</sup> solves are only accepted when using 1D Vision</li>
                </ul>
            </div>

            <SolveForm
                initialValues={createEmptySolveData(user?.id)}
                submitLabel="Submit solve"
                onSubmit={async (value) => {
                    const response = await submitSolve(value)
                    if (response) {
                        navigate(response.redirect)
                    }
                }}
            />
        </>
    )
}

export default SubmitSolve
