import { useSearchParams } from "react-router-dom"
import { Button } from "./ui/button"
import { ButtonGroup } from "./ui/button-group"
import { Card, CardContent } from "./ui/card"
import { Field, FieldLabel } from "./ui/field"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "./ui/select"
import { useEffect, useState } from "react"
import { getCombinedVariants, type CombinedVariant } from "@/lib/backend"

interface props {
    puzzleId?: number
}

/**
 * Widget for category query that sits above the solve table on pages.
 * @param puzzleId - ID of the puzzle when viewing a puzzle page
 */
function CategoryQuery({puzzleId}: props) {

    const [variants, setVariants] = useState<CombinedVariant[]>([])

    const [macros, setMacros] = useState<boolean|null>(null)
    const [filters, setFilters] = useState<boolean|null>(null)

    // handle query parameters
    const [searchParams, setSearchParams] = useSearchParams();

    const [selectedEvent, setSelectedEvent] = useState("single")

    const handleQueryChange = (newValue: string)=> {
        if (newValue === "single") {
            searchParams.delete("event")
            setSelectedEvent("single")
            setSearchParams(searchParams)
        } else {
            const newParams = new URLSearchParams(searchParams)
            if (newParams !== undefined) {
                newParams.set("event", newValue)
                setSelectedEvent(newValue)
                setSearchParams(newParams)
            }

        }

    }

    useEffect(()=> {
        const searchQuery = searchParams.get("event")
        if (searchQuery !== null) {
            setSelectedEvent(searchQuery)
        }
        if (puzzleId) {
            getCombinedVariants(puzzleId).then(setVariants)
        }
    }, [])

    return (
        <Card className="mb-2">
            <CardContent>
                <div className="flex">

                    <Field>
                        <FieldLabel>Event</FieldLabel>
                        <Select value={selectedEvent} onValueChange={handleQueryChange}>
                            <SelectTrigger className="max-w-1/2">
                                <SelectValue placeholder="Single speedsolve" />
                            </SelectTrigger>
                            <SelectContent position="popper">
                                <SelectGroup>
                                    <SelectLabel>Speed</SelectLabel>
                                    <SelectItem value="single">Single speedsolve</SelectItem>
                                    <SelectItem value="avg">Average speedsolve</SelectItem>
                                    <SelectItem value="bld">Blindfolded</SelectItem>
                                    <SelectItem value="oh">One-handed</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Fewest moves</SelectLabel>
                                    <SelectItem value="fmc">Fewest moves</SelectItem>
                                    <SelectItem value="fmcca">Computer-assisted</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Aggregate</SelectLabel>
                                    <SelectItem value="distinct">Distinct Puzzles</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </Field>

                    <Field>
                        <FieldLabel>Piece filters allowed</FieldLabel>
                        <ButtonGroup>
                            <Button variant={filters == null ? "default" : "secondary"} onClick={() => setFilters(null)}>Default</Button>
                            <Button variant={filters == false ? "default" : "secondary"} onClick={() => setFilters(false)}>No</Button>
                            <Button variant={filters == true ? "default" : "secondary"} onClick={() => setFilters(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>


                    <Field>
                        <FieldLabel>Macros allowed</FieldLabel>
                        <ButtonGroup>
                            <Button variant={macros == null ? "default" : "secondary"} onClick={() => setMacros(null)}>Default</Button>
                            <Button variant={macros == false ? "default" : "secondary"} onClick={() => setMacros(false)}>No</Button>
                            <Button variant={macros == true ? "default" : "secondary"} onClick={() => setMacros(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>

                    {puzzleId !== undefined ?
                        <>
                            <Field>
                                <FieldLabel>Variant</FieldLabel>
                                <ButtonGroup>
                                    {variants && variants.map((variant) => (
                                        <Button>{variant.name}</Button>
                                    ))}

                                </ButtonGroup>
                            </Field>

                            <Field>
                                <FieldLabel>Listing</FieldLabel>
                                <ButtonGroup>
                                    <Button>Current rankings</Button>
                                    <Button>Record history</Button>
                                </ButtonGroup>
                            </Field>
                        </>
                        :
                        <>
                        </>
                    }



                </div>

            </CardContent>
        </Card>
    )
}

export default CategoryQuery
