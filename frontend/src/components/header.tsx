import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu"
import { Button } from "@/components/ui/button"
import User from "@/components/user"
import { BookOpen, CirclePlus, CircleQuestionMark, ListOrdered, Podium } from "lucide-react"
import { Link, useNavigate } from "react-router-dom"
import Discord from "./icon/discord"

interface props {
    PageTitle?: string
    excludeBreadcrumbs?: boolean
    hideSubmit?: boolean
}

function Header({PageTitle, hideSubmit}: props) {
    const navigate = useNavigate()
    return (
        <>
            <header className="flex flex-col md:mb-10">
                <a className="text-sidebar-primary hover:underline text-2xl md:text-4xl text-center md:text-left mt-5 mb-5" href="/">Hypercubing Leaderboards</a>
                <NavigationMenu>
                    <NavigationMenuList className="flex-wrap">
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
                        <a href="https://hypercubing.xyz/discord/"><Button><Discord className="text-white"/> Discord</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <User />
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </header>
            {PageTitle && (
                <div className="flex flex-col md:flex-row">
                    <h1 className="text-4xl">{PageTitle}</h1>
                    {!hideSubmit && (
                        <div className="md:ml-auto flex items-center justify-center gap-3">
                        
                            <Link to="https://hypercubing.xyz/leaderboard-rules/" >
                                <Button variant="secondary"><ListOrdered/> Rules</Button>
                            </Link>
                            <Link to="/submit-solve" >
                                <Button><CirclePlus/> Submit Solve</Button>
                            </Link>
                        </div>
                        
                    )}
                </div>
            )}
            {/* {PageTitle && <h1 className="text-4xl">{PageTitle}</h1>}
            {!hideSubmit && (
                <div className="flex justify-center my-4">
                    <Button onClick={() => navigate("/submit-solve")}>Submit Solve</Button>
                </div>
            )} */}
        </>
    )
}

export default Header
