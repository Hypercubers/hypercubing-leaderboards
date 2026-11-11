import EditProgram from "@/components/edit-program"
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { getPrograms, getPuzzles, getVariants, type Program, type Variant, type Puzzle } from "@/lib/backend"
import { AppWindow, CirclePlus, LayoutGrid, Pencil, Puzzle as PuzzleIcon } from "lucide-react"
import { useEffect, useState } from "react"


function Categories() {

    const [puzzles, setPuzzles] = useState<Puzzle[]>([])
    const [programs, setPrograms] = useState<Program[]>([])
    const [variants, setVariants] = useState<Variant[]>([])

    useEffect(() => {
        getPrograms().then(setPrograms)
        getPuzzles().then(setPuzzles)
        getVariants().then(setVariants)
    }, [])



    return (
        <>
            <Header PageTitle="Categories"/>

            <p>BE VERY CAREFUL ON THIS PAGE. THERE IS NO UNDO BUTTON.</p>
            <p>If you need to delete any of these, contact a database administrator.</p>

            <Tabs className="mt-4" defaultValue="puzzles">
                <TabsList >
                    <TabsTrigger value="puzzles"><PuzzleIcon/> Puzzles</TabsTrigger>
                    <TabsTrigger value="programs"><AppWindow/>Programs</TabsTrigger>
                    <TabsTrigger value="variants"><LayoutGrid/> Variants</TabsTrigger>
                </TabsList>

                <TabsContent value="puzzles">
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button><CirclePlus/> Add puzzle</Button>
                        </DialogTrigger>
                        <DialogContent>
                            {/* <EditProgram add={true}/> */}
                        </DialogContent>
                    </Dialog>

                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>ID</TableHead>
                                <TableHead>Name</TableHead>
                                <TableHead>HSC ID</TableHead>
                                <TableHead>Primary Category</TableHead>
                                <TableHead>Autoverifiable</TableHead>
                                <TableHead>Edit</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {puzzles && puzzles.sort((a, b) => a.id - b.id).map((puzzle) => (
                                <TableRow className="*:p-2" key={puzzle.id}>
                                    <TableCell>{puzzle.id}</TableCell>
                                    <TableCell>{puzzle.name}</TableCell>
                                    <TableCell>{puzzle.hsc_id}</TableCell>
                                    <TableCell>{puzzle.primary_filters.toString()}</TableCell>
                                    <TableCell>{puzzle.autoverifiable ? "Yes" : "No"}</TableCell>
                                    <Dialog>
                                        <DialogTrigger asChild>
                                            <Button variant="secondary"><Pencil/> Edit</Button>
                                        </DialogTrigger>
                                        <DialogContent>
                                            {/* <EditProgram add={true}/> */}
                                        </DialogContent>
                                    </Dialog>
                                    
                                </TableRow>
                            ))}
                            
                            {/* </TableCell> */}
                        </TableBody>
                    </Table>
                </TabsContent>

                <TabsContent value="programs">
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button><CirclePlus/> Add program</Button>
                        </DialogTrigger>
                        <DialogContent>
                            <EditProgram add={true}/>
                        </DialogContent>
                    </Dialog>

                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>ID</TableHead>
                                <TableHead>Name</TableHead>
                                <TableHead>Abbreviation</TableHead>
                                <TableHead>Material</TableHead>
                                <TableHead>Edit</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {programs && programs.sort((a, b) => a.id - b.id).map((program) => (
                                <TableRow className="*:p-2" key={program.id}>
                                    <TableCell>{program.id}</TableCell>
                                    <TableCell>{program.name}</TableCell>
                                    <TableCell className="hidden md:inline-flex items-center gap-2"><ProgramIcon abbr={program.abbr} /> {program.abbr}</TableCell>
                                    <TableCell><Checkbox checked={program.material} disabled /></TableCell>
                                    <TableCell>
                                        <Dialog>
                                            <DialogTrigger asChild>
                                                <Button variant="secondary"><Pencil/> Edit</Button>
                                            </DialogTrigger>
                                            <DialogContent>
                                                <EditProgram add={false} fillValues={program}/>
                                            </DialogContent>
                                        </Dialog>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </TabsContent>

            </Tabs>
        </>
    )
}
        
export default Categories