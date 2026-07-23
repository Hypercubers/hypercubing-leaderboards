import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList } from "@/components/ui/combobox"
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "@/components/ui/select"
import { getPuzzles, type Puzzle } from "@/lib/backend"
import { useEffect, useState } from "react"


const variants = [
    { label: "Default", value:"default"},
    { label: "1D Vision", value:"1dvision"},
    { label: "Physical", value:"physical"}
]


function SubmitSolve() {


    // gets a list of puzzles to use for the select puzzle dropdown
    const [puzzles, setPuzzles] = useState<Puzzle[]>([])

    useEffect(() => {
        getPuzzles().then(setPuzzles)
    }, [])




    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">Submit Solve</h1>

            <Card>
                <CardContent>
                    <p>Read the <a className="text-sidebar-primary underline" href="https://hypercubing.xyz/leaderboards/rules/">Hypercubing Leaderboard Submission Rules</a> before submitting. Additionally, note that: </p>
                    <ul className="list-disc pt-4 pl-6">
                        <li>Average of 5 submissions are only accepted for speedsolves of 3<sup>4</sup> (virtual), 2<sup>4</sup> (virtual and physical), and 2<sup>5</sup> (virtual)</li>
                        <li>3<sup>3</sup> solves are only accepted when using 1D Vision</li>
                    </ul>
                </CardContent>
            </Card>

            <form>
                <div className="grid md:grid-cols-2 mt-4 gap-4">
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl" >Event</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <FieldGroup>
                                <Field>
                                    <FieldLabel htmlFor="email">Puzzle</FieldLabel>
                                    <Combobox
                                    items={puzzles}
                                    filter={(item: Puzzle, query, itemToString) => {
                                        const normalize = (value: string) =>
                                            value.replaceAll("×", "x").toLowerCase()
                                        const label = itemToString?.(item) ?? item.name
                                        return normalize(label).includes(normalize(query))
                                    }}
                                    >
                                        <ComboboxInput placeholder="Select a puzzle" />
                                        <ComboboxContent>
                                            <ComboboxEmpty>No puzzles found</ComboboxEmpty>
                                            <ComboboxList>
                                                {(item) => (
                                                    <ComboboxItem key={item.id} value={(item.name)}>{(item.name)}</ComboboxItem>
                                                )}
                                            </ComboboxList>
                                        </ComboboxContent>
                                    </Combobox>
                                </Field>

                                <Field>
                                    <FieldLabel htmlFor="variant">Variant</FieldLabel>

                                    <Select>
                                        <SelectTrigger id="variant" className="w-full max-w-48">
                                            <SelectValue placeholder="Select a variant" />
                                        </SelectTrigger>
                                        <SelectContent position="popper">
                                                <SelectGroup>
                                                    <SelectLabel>Variant</SelectLabel>
                                                    <SelectItem aria-selected value="default">Default</SelectItem>
                                                    <SelectItem value="physical">Physical</SelectItem>
                                                    <SelectItem value="1dvision">1D Vision</SelectItem>
                                                </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                    <FieldDescription>Visual representation + available moves</FieldDescription>
                                </Field>

                                <Field>
                                    <FieldLabel htmlFor="program">Computer program</FieldLabel>

                                    <Select>
                                        <SelectTrigger id="program" className="w-full max-w-48">
                                            <SelectValue placeholder="Select a program" />
                                        </SelectTrigger>
                                        <SelectContent position="popper">
                                                <SelectGroup>
                                                    <SelectLabel>Program</SelectLabel>
                                                    <SelectItem aria-selected value="default">Default</SelectItem>
                                                    <SelectItem value="physical">Physical</SelectItem>
                                                    <SelectItem value="1dvision">1D Vision</SelectItem>
                                                </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                    <FieldDescription>Select “N/A” for solves done without using a computer</FieldDescription>
                                </Field>
                            </FieldGroup>

                        </CardContent>

                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Speedsolve</CardTitle>
                        </CardHeader>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Metadata</CardTitle>
                        </CardHeader>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Fewest moves</CardTitle>
                        </CardHeader>
                    </Card>
                </div>
                <div className="flex mt-4 w-full">
                    <Button className="w-1/2" disabled type="submit">Submit solve</Button>
                </div>


            </form>
        </>
    )
}

export default SubmitSolve
