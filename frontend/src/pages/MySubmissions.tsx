import Header from "@/components/header"
import SearchableDropdown from "@/components/searchable-dropdown"
import { Card, CardContent } from "@/components/ui/card"
import { Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList } from "@/components/ui/combobox"
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "@/components/ui/select"
import { getPuzzles, type Puzzle } from "@/lib/backend"
import { useEffect, useState } from "react"


const variants = [
    { label: "Default", value:"default"},
    { label: "1D Vision", value:"1dvision"},
    { label: "Physical", value:"physical"}
]


function MySubmissions() {


    const [puzzles, setPuzzles] = useState<Puzzle[]>([])

    const puzzleInfo = [...puzzles]

    useEffect(() => {
        getPuzzles().then(setPuzzles)
    }, [])

    function replaceTimesToX(str: string) {
        return str.replaceAll("×", "x")
    }


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
                <div className="grid md:grid-cols-2">
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
                            {/* <Select>
                                <SelectTrigger id="email" className="w-full max-w-48">
                                    <SelectValue placeholder="Select a puzzle" />
                                </SelectTrigger>
                                <SelectContent position="popper">

                                    <SelectGroup>
                                        <SelectLabel>Puzzle</SelectLabel>
                                        { puzzles && puzzles.length > 0 ? puzzles?.map((puzzle: Puzzle) => (
                                                <SelectItem value={puzzle.name}>{puzzle.name}</SelectItem>
                                        )
                                        ):
                                        (
                                            <></>
                                        )
                                        }

                                    </SelectGroup>
                                </SelectContent>
                            </Select> */}
                            {/* <Input
                            id="email"
                            type="email"
                            placeholder="support@hypercubing.xyz"
                            required
                            /> */}
                        </Field>
                    </FieldGroup>

                </div>

            </form>
        </>
    )
}

export default MySubmissions
