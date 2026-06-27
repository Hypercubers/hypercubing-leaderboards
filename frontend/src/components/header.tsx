// import { useState } from 'react'

import { Button } from "./ui/button"
import User from "@/components/user"

function Header() {
    return (
        <>
            <header className="flex flex-col">
                <h1 className="text-4xl m-2"><a href="/">Hypercubing Leaderboards</a></h1>
                <nav>
                    <div className="flex gap-3">
                        <Button><a href="https://hypercubing.xyz/">Wiki</a></Button>
                        <Button><a href="https://hypercubing.xyz/faq/">FAQ</a></Button>
                        <Button><a href="/">Leaderboards</a></Button>
                        <Button><a href="https://hypercubing.xyz/discord/">Discord</a></Button>
                        <User/>
                    </div>


                </nav>
            </header>
        </>
    )
}

export default Header
