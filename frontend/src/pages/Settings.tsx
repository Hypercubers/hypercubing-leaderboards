import Header from "@/components/header"
import Discord from "@/components/icon/discord"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Item, ItemContent, ItemDescription, ItemMedia, ItemTitle } from "@/components/ui/item"
import { useAuth } from "@/lib/auth-context"
import { AtSign, Lock, UserRound } from "lucide-react"



function Settings() {
    const { user } = useAuth()


    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Settings</h1>

            <div className="flex flex-col gap-4">
                <Item variant={"outline"}>
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
                    <Item variant="outline">
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
                    <Item variant="outline">
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

                <Item variant="outline">
                    <ItemMedia>
                        <Lock/>
                    </ItemMedia>
                    <ItemContent>
                        <ItemTitle>Security</ItemTitle>
                        <ItemDescription>
                            <Button>Sign out everywhere</Button>
                        </ItemDescription>
                    </ItemContent>
                </Item>
            </div>
        </>
    )
}

export default Settings
