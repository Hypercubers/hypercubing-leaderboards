import Header from "@/components/header"
import Discord from "@/components/icon/discord"
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from "@/components/ui/alert-dialog"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Item, ItemContent, ItemDescription, ItemMedia, ItemTitle } from "@/components/ui/item"
import { useAuth } from "@/lib/auth-context"
import { updateName } from "@/lib/backend"
import { AtSign, Lock, UserRound } from "lucide-react"
import { useState } from "react"
import { useNavigate } from "react-router-dom"



function Settings() {
    const { user, signOutEverywhere, refreshUser } = useAuth()
    const [newName, setNewName] = useState('')
    const navigate = useNavigate()

    async function handleSignOut() {
        const redirect = await signOutEverywhere()
        navigate(redirect, {replace: true})
    }

    async function handleNameChange(event: React.SubmitEvent) {
        event.preventDefault()
        const redirect = await updateName(user?.id, newName)
        refreshUser()
        setNewName('')
        navigate(redirect ?? "/settings")
    }


    return (
        <>
            <Header PageTitle="Settings"/>

            <div className="flex flex-col gap-4 m-2">
                <Item variant="muted">
                    <ItemMedia>
                        <UserRound className="size-5" />
                    </ItemMedia>
                    <ItemContent>
                        <ItemTitle>Display name</ItemTitle>
                            <form className="flex gap-2" onSubmit={handleNameChange}>
                                <Input
                                type="text"
                                placeholder={user?.name || "John Hypercubing"}
                                value={newName}
                                onChange={(event) => setNewName(event.target.value)}
                                />
                                <Button type="submit" disabled={!newName}>Update</Button>
                            </form>
                    </ItemContent>
                </Item>

                {user?.email &&
                    <Item variant="muted">
                        <ItemMedia>
                            <AtSign className="size-5" />
                        </ItemMedia>
                        <ItemContent>
                            <ItemTitle>Email</ItemTitle>
                            <ItemDescription>{user?.email || "no email found"}</ItemDescription>
                        </ItemContent>
                    </Item>
                }

                {user?.discord_id &&
                    <Item variant="muted">
                        <ItemMedia variant="icon">
                            <Discord/>
                        </ItemMedia>
                        <ItemContent>
                            <ItemTitle>Discord account</ItemTitle>
                            <ItemDescription>
                                <div className="flex gap-2">
                                    <Avatar size="lg">
                                        <AvatarImage src={user?.discord_avatar_url} />
                                        <AvatarFallback>{user?.name?.charAt(0)}</AvatarFallback>
                                    </Avatar>
                                    <div>
                                        <p className="font-bold">{user?.discord_nickname || "no email found"}</p>
                                        <p>{user?.discord_username || "no email found"}</p>
                                    </div>
                                </div>
                            </ItemDescription>
                        </ItemContent>
                    </Item>
                }

                <Item variant="muted">
                    <ItemMedia>
                        <Lock/>
                    </ItemMedia>
                    <ItemContent>
                        <ItemTitle>Security</ItemTitle>
                        <ItemDescription>
                            <AlertDialog>
                                <AlertDialogTrigger asChild>
                                    <Button>Sign out everywhere</Button>
                                </AlertDialogTrigger>
                                <AlertDialogContent>
                                    <AlertDialogHeader>
                                        <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                                        <AlertDialogDescription>This action will log you out across all tabs, browsers, and devices.</AlertDialogDescription>
                                    </AlertDialogHeader>
                                    <AlertDialogFooter>
                                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                                        <AlertDialogAction variant="destructive" onClick={() => void handleSignOut()}>Sign out</AlertDialogAction>
                                    </AlertDialogFooter>
                                </AlertDialogContent>
                            </AlertDialog>
                        </ItemDescription>
                    </ItemContent>
                </Item>
            </div>
        </>
    )
}

export default Settings
