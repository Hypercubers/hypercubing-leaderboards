import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field"
import { InputOTP, InputOTPGroup, InputOTPSlot } from "@/components/ui/input-otp"
import { submitOtpRequest } from "@/lib/backend"
import { useAuth } from "@/lib/auth-context"
import { REGEXP_ONLY_DIGITS } from "input-otp"
import { useState, type SubmitEvent } from "react"
import { useLocation, useNavigate } from "react-router-dom"

function RequestOtpDiscord() {

    const navigate = useNavigate()
    const location = useLocation()
    const { refreshUser } = useAuth()

    const [value, setValue] = useState("")
    const deviceCode = (location.state as { deviceCode?: string } | null)?.deviceCode

    const submitDisabled = !(value.length==6)

    async function verifyOtp(e: SubmitEvent<HTMLFormElement>) {
        e.preventDefault()

        if (!deviceCode) {
            navigate("/sign-in")
            return
        }

        const redirectPath = await submitOtpRequest({
            device_code: deviceCode,
            otp: value,
        })

        if (redirectPath != null) {
            await refreshUser()
            navigate(redirectPath)
        }
    }

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Verify</h1>

            <div className="flex justify-center align-center *:p-4">
                <Card className="w-full max-w-md">
                    <CardHeader>
                        <CardTitle>Verify your login</CardTitle>
                        <CardDescription>Check your Discord DMs for a one-time code, then copy and paste it here.</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <form onSubmit={verifyOtp}>
                            <FieldGroup>
                                <Field>
                                    <FieldLabel>Verification code</FieldLabel>
                                    <InputOTP value={value}
                                    maxLength={6}
                                    pattern={REGEXP_ONLY_DIGITS}
                                    onChange={(value) => setValue(value)}
                                    >
                                    <InputOTPGroup className="*:data-[slot=input-otp-slot]:size-12 *:data-[slot=input-otp-slot]:text-xl">
                                        <InputOTPSlot index={0} />
                                        <InputOTPSlot index={1} />
                                        <InputOTPSlot index={2} />
                                        <InputOTPSlot index={3} />
                                        <InputOTPSlot index={4} />
                                        <InputOTPSlot index={5} />
                                    </InputOTPGroup>
                                    </InputOTP>
                                </Field>

                                <Field>
                                    <Button disabled={submitDisabled} type="submit">Verify</Button>
                                </Field>
                            </FieldGroup>
                        </form>

                    </CardContent>
                    <CardFooter>
                        <p>If you can't find the message, you can try again or <a className="text-sidebar-primary underline" href="mailto:support@hypercubing.xyz">email us for help</a>.</p>

                    </CardFooter>
                </Card>
            </div>
        </>

    )
}

export default RequestOtpDiscord
