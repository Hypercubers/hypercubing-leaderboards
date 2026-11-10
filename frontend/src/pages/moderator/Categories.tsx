import Header from "@/components/header"
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { AppWindow, LayoutGrid, Puzzle } from "lucide-react"


function Categories() {
    return (
        <>
            <Header PageTitle="Categories"/>

            <p>BE VERY CAREFUL ON THIS PAGE. THERE IS NO UNDO BUTTON.</p>
            <p>If you need to delete any of these, contact a database administrator.</p>

            <Tabs className="items-center" defaultValue="puzzles">
                <TabsList >
                    <TabsTrigger value="puzzles"><Puzzle/> Puzzles</TabsTrigger>
                    <TabsTrigger value="programs"><AppWindow/>Programs</TabsTrigger>
                    <TabsTrigger value="variants"><LayoutGrid/> Variants</TabsTrigger>
                </TabsList>
            </Tabs>
        </>
    )
}

export default Categories