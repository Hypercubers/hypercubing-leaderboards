import EditProgram from "@/components/edit-program"
import EditVariant from "@/components/edit-variant"
import Header from "@/components/header"
import ProgramIcon from "@/components/icon/program-icon"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Field } from "@/components/ui/field"
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
                                <TableHead>Edit</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {puzzles && puzzles.sort((a, b) => a.id - b.id).map((puzzle) => (
                                <TableRow className="*:p-2" key={puzzle.id}>
                                    <TableCell>{puzzle.id}</TableCell>
                                    <TableCell>{puzzle.name}</TableCell>
                                    <TableCell>{puzzle.hsc_id}</TableCell>
                                    <TableCell className="inline-flex items-center gap-2">
                                        <Field orientation="horizontal">
                                            <Checkbox checked={puzzle.primary_filters}/> Allow filters
                                            <Checkbox checked={puzzle.primary_macros}/> Allow macros
                                            <Checkbox checked={puzzle.autoverifiable} /> Autoverifiable
                                        </Field>
                                    </TableCell>
                                    <TableCell>
                                        <Dialog>
                                            <DialogTrigger asChild>
                                                <Button variant="secondary"><Pencil/> Edit</Button>
                                            </DialogTrigger>
                                            <DialogContent>
                                                {/* <EditProgram add={true}/> */}
                                            </DialogContent>
                                        </Dialog>
                                    </TableCell>
                                    
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
                                    <TableCell className="inline-flex items-center gap-2"><ProgramIcon abbr={program.abbr} /> {program.abbr}</TableCell>
                                    <TableCell>
                                        <Field orientation="horizontal">
                                            <Checkbox checked={program.material}/>Material
                                        </Field>
                                    </TableCell>
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

                <TabsContent value="variants">
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button><CirclePlus/> Add variant</Button>
                        </DialogTrigger>
                        <DialogContent>
                            <EditVariant add={true}/>
                        </DialogContent>
                    </Dialog>

                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>ID</TableHead>
                                <TableHead>Name</TableHead>
                                <TableHead>Prefix</TableHead>
                                <TableHead>Suffix</TableHead>
                                <TableHead>Abbreviation</TableHead>
                                <TableHead>Primary Category</TableHead>
                                <TableHead>Edit</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {variants && variants.sort((a, b) => a.id - b.id).map((variant) => (
                                <TableRow className="*:p-2" key={variant.id}>
                                    <TableCell>{variant.id}</TableCell>
                                    <TableCell>{variant.name}</TableCell>
                                    <TableCell>{variant.prefix}</TableCell>
                                    <TableCell>{variant.suffix}</TableCell>
                                    <TableCell>{variant.abbr}</TableCell>
                                    <TableCell>
                                        <Field orientation="horizontal">
                                            <Checkbox checked={variant.material_by_default}/> Material
                                            <Checkbox checked={variant.primary_filters}/> Allow filters
                                            <Checkbox checked={variant.primary_macros}/> Allow macros
                                        </Field>
                                    </TableCell>
                                    <TableCell>
                                        <Dialog>
                                            <DialogTrigger asChild>
                                                <Button variant="secondary"><Pencil/> Edit</Button>
                                            </DialogTrigger>
                                            <DialogContent>
                                                <EditVariant add={false} fillValues={variant}/>
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