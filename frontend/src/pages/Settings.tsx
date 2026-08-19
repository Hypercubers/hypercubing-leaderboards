import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"


function Settings() {
    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Settings</h1>

            <h2 className="text-xl" m-2>Account</h2>
            <Card>
                <CardHeader>
                    <CardTitle>Display name</CardTitle>
                </CardHeader>
                <CardContent>
                    <Input></Input>
                </CardContent>
            </Card>

            <h2 className="text-xl" m-2>Security</h2>
            <Card>
                <CardHeader>
                    <CardTitle>Log ins</CardTitle>
                </CardHeader>
                <CardContent>
                    <Button>Sign out everywhere</Button>
                </CardContent>
            </Card>
        </>
    )
}

export default Settings
