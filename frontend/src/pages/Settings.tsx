import Header from "@/components/header"
import Discord from "@/components/icon/discord"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Item, ItemContent, ItemDescription, ItemMedia, ItemTitle } from "@/components/ui/item"
import { useAuth } from "@/lib/auth-context"
import { AtSign, Lock, UserRound } from "lucide-react"
import { useNavigate } from "react-router-dom"



function Settings() {
    const { user, signOutEverywhere  } = useAuth()
    const navigate = useNavigate()

    async function handleSignOut() {
        const redirect = await signOutEverywhere()
        navigate(redirect, {replace: true})
    }


    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Settings</h1>

            <div className="flex flex-col gap-4">
                <Item variant="muted">
                    <ItemMedia>
                        <UserRound className="size-5" />
                    </ItemMedia>
                    <ItemContent>
                        <ItemTitle>Display name</ItemTitle>
                        <div className="flex gap-2">
                            <Input placeholder={user?.name || "John Hypercubing"}></Input>
                            <Button>Update</Button>
                        </div>
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
                            <Button onClick={() => void handleSignOut()}>Sign out everywhere</Button>
                        </ItemDescription>
                    </ItemContent>
                </Item>
            </div>
        </>
    )
}

export default Settings
