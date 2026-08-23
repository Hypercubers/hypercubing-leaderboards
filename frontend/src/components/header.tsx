import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu"
import { Button } from "@/components/ui/button"
import User from "@/components/user"
import { BookOpen, CircleQuestionMark, Podium } from "lucide-react"
import { Link, useNavigate } from "react-router-dom"
import Discord from "./icon/discord"
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from "./ui/breadcrumb"

interface props {
    PageTitle?: string
    excludeBreadcrumbs?: boolean
}

function Header({PageTitle, excludeBreadcrumbs}: props) {
    const navigate = useNavigate()
    return (
        <>
            <header className="flex flex-col md:mb-10">
                <a className="text-sidebar-primary hover:underline text-2xl md:text-4xl text-center md:text-left mt-5 mb-5" href="https://lb.hypercubing.xyz/">Hypercubing Leaderboards</a>
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
            {PageTitle && <h1 className="text-4xl m-2">{PageTitle}</h1>}
            {/* {!excludeBreadcrumbs &&
                <Breadcrumb className="mb-4 ml-2">
                    <BreadcrumbList>
                        <BreadcrumbItem>
                        <BreadcrumbLink asChild>
                            <Link to="/">World Records</Link>
                        </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                        <BreadcrumbPage>{PageTitle}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            } */}
        </>
    )
}

export default Header
