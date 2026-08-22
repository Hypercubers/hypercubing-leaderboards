import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu"
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
                <a className="text-sidebar-primary hover:underline text-4xl mt-5 mb-5" href="https://lb.hypercubing.xyz/">Hypercubing Leaderboards</a>
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
                        <User />
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </header>
        </>
    )
}

export default Header
