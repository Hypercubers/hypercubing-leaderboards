import Header from "@/components/header"
import Discord from "@/components/icon/discord"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { AtSign } from "lucide-react"

import { Turnstile } from '@marsidev/react-turnstile'
import { requestOtpDiscord, type SignInDiscordRequest } from "@/lib/backend"
import { useState, type SubmitEvent } from "react"
import { useNavigate } from "react-router-dom"




function SignIn() {
    const [discordUsername, setDiscordUsername] = useState("")
    const [turnstileResponse, setTurnstileResponse] = useState<string | undefined>()
    const navigate = useNavigate()

    async function handleDiscordSignIn(e: SubmitEvent<HTMLFormElement>) {
        e.preventDefault()
        const data: SignInDiscordRequest = {
            username: discordUsername,
            turnstile_response: turnstileResponse,
        }
        const response = await requestOtpDiscord(data)

        if (response != null) {
            navigate("/request-otp-discord", {
                state: { deviceCode: response.device_code },
            })
        }
    }

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Sign In</h1>

            <div className="flex justify-center align-center *:p-4">

                    <Tabs className="items-center" defaultValue="discord">
                        <FieldDescription>
                            <p>If you have already created an account using one option, do not use the other option.</p>
                        </FieldDescription>
                        <TabsList >
                            <TabsTrigger value="discord"><Discord/> Discord</TabsTrigger>
                            <TabsTrigger value="email"><AtSign/> Email</TabsTrigger>
                        </TabsList>

                        {/* email sign in card */}
                        <TabsContent value="email">
                            <Card className="w-full max-w-md">
                                <CardHeader>
                                    <CardTitle>Sign in with email</CardTitle>
                                    <CardDescription>
                                        Enter your email below to login to your account
                                    </CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <form>
                                        <FieldGroup>
                                            <Field>
                                                <FieldLabel htmlFor="email">Email</FieldLabel>
                                                <Input
                                                id="email"
                                                type="email"
                                                placeholder="support@hypercubing.xyz"
                                                required
                                                />
                                            </Field>
                                            <Field>
                                                <Button type="submit">Send code</Button>
                                            </Field>
                                        </FieldGroup>
                                    </form>
                                </CardContent>
                                <CardFooter className="flex flex-col border-t *:pt-2 *:text-muted-foreground">
                                    <p>This will send an email to your address with a one-time passcode that you can use to log in.</p>
                                    <p>Your email address will not be visible on the leaderboards, but may be used by leaderboard staff to contact you about your submissions.</p>
                                    <p>Signing in will create an account if there isn't already one for your email address.</p>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                        {/* Discord sign in card */}
                        <TabsContent value="discord">
                            <Card className="w-full max-w-md">
                                <CardHeader>
                                    <CardTitle>Sign in with Discord</CardTitle>
                                    <CardDescription>
                                        Enter your username below to login to your account
                                    </CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <form onSubmit={handleDiscordSignIn}>
                                        <FieldGroup>
                                            <Field>
                                                <FieldLabel htmlFor="discord">Username</FieldLabel>
                                                <Input
                                                id="discord"
                                                type="text"
                                                placeholder="username123"
                                                value={discordUsername}
                                                onChange={(event) => setDiscordUsername(event.target.value)}
                                                required
                                                />
                                            </Field>
                                            <Field>
                                                <Button type="submit">Send code</Button>
                                            </Field>
                                        </FieldGroup>
                                    </form>
                                </CardContent>
                                <CardFooter className="flex flex-col border-t *:pt-2 *:text-muted-foreground">
                                    <p>This requires you to join the Hypercubers Discord server in order to receive a direct message from the Hypercubers bot. The bot will send you a one-time passcode that you can use to log in.</p>
                                    <p>Your Discord account will not be visible on the leaderboards, but it will be visible to anyone else on the Hypercubers Discord server and may be used by leaderboard staff to contact you about your submissions.</p>
                                    <p>Signing in will create an account if there isn't already one for your Discord account.</p>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                        <FieldDescription>
                            <p>Trouble signing in? Ask on the <a className="text-sidebar-primary underline" href="https://hypercubing.xyz/discord/">Hypercubers Discord server</a> or email <a className="text-sidebar-primary underline" href="mailto:support@hypercubing.xyz">support@hypercubing.xyz</a>.</p>
                        </FieldDescription>

                        <Turnstile
                            siteKey={import.meta.env.VITE_TURNSTILE_SECRET_KEY}
                            onSuccess={setTurnstileResponse}
                            onExpire={() => setTurnstileResponse(undefined)}
                            onError={() => setTurnstileResponse(undefined)}
                        />

                    </Tabs>



            </div>



        </>
    )

}

export default SignIn
