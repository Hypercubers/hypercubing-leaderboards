// import { useState } from 'react'

import { NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList } from "@/components/ui/navigation-menu"
import { Button } from "@/components/ui/button"
import User from "@/components/user"
import { BookOpen, CircleQuestionMark, Podium } from "lucide-react"
import { useNavigate } from "react-router-dom"
import Discord from "./icon/discord"

function Header() {
    const navigate = useNavigate()
    return (
        <>
            <header className="flex flex-col mb-10">
                <h1 onClick={() => navigate("/")} className="text-4xl mt-5 mb-5">Hypercubing Leaderboards</h1>
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                                <a href="https://hypercubing.xyz/"><Button><BookOpen/> Wiki</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                                <a href="https://hypercubing.xyz/faq/"><Button><CircleQuestionMark/> FAQ</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                                <Button onClick={() => navigate("/")}><Podium/> Leaderboards</Button>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                                <a href="https://hypercubing.xyz/discord/"><Button><Discord/> Discord</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            {/* Show <User/> if logged in */}
                            <Button variant="secondary" onClick={() => navigate("/signin")}>Sign In</Button>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </header>
        </>
    )
}

export default Header
