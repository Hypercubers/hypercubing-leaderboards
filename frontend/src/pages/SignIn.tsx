import Header from "@/components/header"
import Discord from "@/components/icon/discord"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { AtSign } from "lucide-react"



function SignIn() {
    return (
        <>
            {/* <Header/> */}
            <Header/>
            <h1 className="text-4xl m-2">Sign In</h1>
            <div className=" *:p-4">


                    <Tabs defaultValue="discord">
                        <TabsList >
                            <TabsTrigger value="discord"><Discord/> Discord</TabsTrigger>
                            <TabsTrigger value="email"><AtSign/> Email</TabsTrigger>
                        </TabsList>
                        {/* email sign in card */}
                        <TabsContent value="email">
                            <Card className="w-full max-w-sm">
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
                                                placeholder="m@example.com"
                                                required
                                                />
                                            </Field>
                                            <Field>
                                                <Button type="submit">Send code</Button>
                                            </Field>
                                        </FieldGroup>
                                    </form>
                                </CardContent>
                                <CardFooter className="flex flex-col border-t *:pt-2">
                                    <p>This will send an email to your address with a one-time passcode that you can use to log in.</p>
                                    <p>Your email address will not be visible on the leaderboards, but may be used by leaderboard staff to contact you about your submissions.</p>
                                    <p>Signing in will create an account if there isn't already one for your email address.</p>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                        {/* Discord sign in card */}
                        <TabsContent value="discord">
                            <Card className="w-full max-w-sm">
                                <CardHeader>
                                    <CardTitle>Sign in with Discord</CardTitle>
                                    <CardDescription>
                                        Enter your Discord username below to login to your account
                                    </CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <form>
                                        <FieldGroup>
                                            <Field>
                                                <FieldLabel htmlFor="email">Username</FieldLabel>
                                                <Input
                                                id="email"
                                                type="email"
                                                placeholder="m@example.com"
                                                required
                                                />
                                            </Field>
                                            <Field>
                                                <Button type="submit">Send code</Button>
                                            </Field>
                                        </FieldGroup>
                                    </form>
                                </CardContent>
                                <CardFooter className="flex flex-col border-t *:pt-2">
                                    <p>This requires you to join the Hypercubers Discord server in order to receive a direct message from the Hypercubers bot. The bot will send you a direct message with a button to log in.</p>
                                    <p>Your Discord account will not be visible on the leaderboards, but it will be visible to anyone else on the Hypercubers Discord server and may be used by leaderboard staff to contact you about your submissions.</p>
                                    <p>Signing in will create an account if there isn't already one for your Discord account.</p>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                    </Tabs>
            </div>
        </>
    )

}

export default SignIn
