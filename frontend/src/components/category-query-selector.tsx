import { useSearchParams } from "react-router-dom"
import { Button } from "./ui/button"
import { ButtonGroup } from "./ui/button-group"
import { Card, CardContent } from "./ui/card"
import { Field, FieldLabel } from "./ui/field"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "./ui/select"
import { useEffect, useState } from "react"
import { getCombinedVariants, type CategoryQuery, type CombinedVariant } from "@/lib/backend"

interface props {
    puzzleId?: number
}

/**
 * Widget for category query that sits above the solve table on pages.
 * @param puzzleId - ID of the puzzle when viewing a puzzle page
 */
function CategoryQuerySelector({puzzleId}: props) {

    const [variants, setVariants] = useState<CombinedVariant[]>([])
    const [selectedVariant, setSelectedVariant] = useState<String>()

    const [macros, setMacros] = useState<boolean|undefined>(undefined)
    const [filters, setFilters] = useState<boolean|undefined>(undefined)
    const [recordHistory, setRecordHistory] = useState<boolean>(false)
    const [selectedEvent, setSelectedEvent] = useState("single")

    const [searchParams, setSearchParams] = useSearchParams();

    // let query: CategoryQuery = {
    //     Speed: {
    //         average: selectedEvent === "avg",
    //         blind: selectedEvent === "bld",
    //         filters: filters,
    //         macros: macros,
    //         one_handed: selectedEvent === "oh",
    //         variant:
    //     }
    // }

    function handleRecordHistoryChange(state: boolean) {
        setRecordHistory(state)
        if (state) {
            searchParams.set("history", "true")
            setSearchParams(searchParams)
        } else {
            searchParams.delete("history")
            setSearchParams(searchParams)
        }
    }


    function handleEventChange(event: string) {
        setSelectedEvent(event)
        if (event === "single") {
            searchParams.delete("event")
            setSearchParams(searchParams)
        } else {
            searchParams.set("event", event)
            setSearchParams(searchParams)
        }
    }

    function handleMacroChange(state: boolean|undefined) {
        setMacros(state)
        if (state == undefined) {
            searchParams.delete("macros")
            setSearchParams(searchParams)
        } else {
            searchParams.set("macros", state?"true":"false")
            setSearchParams(searchParams)
        }
    }

    function handleFilterChange(state: boolean|undefined) {
        setFilters(state)
        if (state == undefined) {
            searchParams.delete("filters")
            setSearchParams(searchParams)
        } else {
            searchParams.set("filters", state?"true":"false")
            setSearchParams(searchParams)
        }
    }

    // hardcoded for these variants for now (will be changed in the future)
    function handleVariantChange(name: string) {
        setSelectedVariant(name)
        searchParams.delete("variant")
        searchParams.delete("program")
        if (name === "Virtual") {
            setSearchParams(searchParams)
        } else if (name === "Physical") {
            searchParams.set("variant", "phys")
            setSearchParams(searchParams)
        } else if (name === "Virtual Physical") {
            searchParams.set("variant", "phys")
            searchParams.set("program", "virtual")
            setSearchParams(searchParams)
        } else if (name === "Material") {
            searchParams.set("program", "material")
            setSearchParams(searchParams)
        } else if (name === "1D Vision") {
            searchParams.set("variant", "1d")
            setSearchParams(searchParams)
        }
        // else {it won't set anything}
    }

    useEffect(()=> {
        if (puzzleId) {
            getCombinedVariants(puzzleId).then((loadedVariants)=> {
                setVariants(loadedVariants)
                if (loadedVariants.length > 0) {
                    setSelectedVariant(loadedVariants[0].name)
                }
            })
        }
    }, [])

    return (
        <Card className="mb-2">
            <CardContent>
                <div className="flex gap-4">

                    <Field className="w-min">
                        <FieldLabel>Event</FieldLabel>
                        <Select value={selectedEvent} onValueChange={handleEventChange}>
                            <SelectTrigger>
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

                    <Field className="w-min">
                        <FieldLabel>Piece filters allowed</FieldLabel>
                        <ButtonGroup>
                            <Button variant={filters == undefined ? "default" : "secondary"} onClick={() => handleFilterChange(undefined)}>Default</Button>
                            <Button variant={filters == false ? "default" : "secondary"} onClick={() => handleFilterChange(false)}>No</Button>
                            <Button variant={filters == true ? "default" : "secondary"} onClick={() => handleFilterChange(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>


                    <Field className="w-min">
                        <FieldLabel>Macros allowed</FieldLabel>
                        <ButtonGroup>
                            <Button variant={macros == undefined ? "default" : "secondary"} onClick={() => handleMacroChange(undefined)}>Default</Button>
                            <Button variant={macros == false ? "default" : "secondary"} onClick={() => handleMacroChange(false)}>No</Button>
                            <Button variant={macros == true ? "default" : "secondary"} onClick={() => handleMacroChange(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>

                    {puzzleId !== undefined ?
                        <>
                            <Field className="w-min">
                                <FieldLabel>Variant</FieldLabel>
                                <ButtonGroup>
                                    {variants && variants.map((variant) => (
                                        <Button variant={selectedVariant===variant.name? "default" : "secondary"} onClick={()=> handleVariantChange(variant.name)}>{variant.name}</Button>
                                    ))}

                                </ButtonGroup>
                            </Field>

                            <Field className="w-min">
                                <FieldLabel>Listing</FieldLabel>
                                <ButtonGroup>
                                    <Button variant={recordHistory? "secondary" : "default"} onClick={()=> handleRecordHistoryChange(false)}>Current rankings</Button>
                                    <Button variant={recordHistory? "default" : "secondary"} onClick={()=> handleRecordHistoryChange(true)}>Record history</Button>
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

export default CategoryQuerySelector
