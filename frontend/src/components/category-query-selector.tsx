import { useSearchParams } from "react-router-dom"
import { Button } from "./ui/button"
import { ButtonGroup } from "./ui/button-group"
import { Card, CardContent } from "./ui/card"
import { Field, FieldLabel } from "./ui/field"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "./ui/select"
import { useEffect, useState } from "react"
import { getCombinedVariants, type CategoryQuery, type CombinedVariant } from "@/lib/backend"
import { EyeOff, Hand, Laptop, ScrollText, Shapes, Sigma, Timer } from "lucide-react"

const FIXED_VARIANT_OPTIONS = [
    { label: "Virtual", variant: "Default", program: "Default" },
    { label: "Physical", variant: "phys", program: "Default" },
    { label: "Virtual Physical", variant: "phys", program: "virtual" },
    { label: "Material", variant: "Default", program: "material" },
    { label: "1D Vision", variant: "1d", program: "Default" },
]

interface props {
    puzzleId?: number
    query: CategoryQuery
}

/**
 * Widget for category query that sits above the solve table on pages.
 * @param puzzleId - ID of the puzzle when viewing a puzzle page
 * @param query - the category query
 */
function CategoryQuerySelector({puzzleId, query}: props) {

    const [recordHistory, setRecordHistory] = useState<boolean>(false)
    const [searchParams, setSearchParams] = useSearchParams();
    const [combinedVariants, setCombinedVariants] = useState<CombinedVariant[]>([])

    useEffect(() => {
        if (puzzleId == undefined) return

        getCombinedVariants(puzzleId).then((variants) => {
            setCombinedVariants(variants)
        })
    }, [puzzleId])

    const availableVariantOptions = combinedVariants.length > 0
        ? FIXED_VARIANT_OPTIONS.filter((option) => combinedVariants.some((combined) => combined.name === option.label))
        : FIXED_VARIANT_OPTIONS

    const selectedEvent = query.distinct ? "distinct" :
    query.Fmc.enabled ? (query.Fmc.computer_assisted ? "fmcca" : "fmc")
        : query.Speed.average ? "avg"
            : query.Speed.blind ? "bld"
                : query.Speed.one_handed ? "oh"
                    : "single"

    const selectedVariant = query.Speed.variant
    const selectedProgram = query.Speed.program
    const filters = query.Speed.filters
    const macros = query.Speed.macros

    function syncUrl(nextQuery: CategoryQuery) {
        const nextParams = new URLSearchParams(searchParams.toString())


        if (nextQuery.distinct) {
            nextParams.set("event", "distinct")
            nextParams.delete("filters")
            nextParams.delete("macros")
            nextParams.delete("variant")
            nextParams.delete("program")
        } else {
            if (nextQuery.Speed.average) nextParams.set("event", "avg")
            else if (nextQuery.Speed.blind) nextParams.set("event", "bld")
            else if (nextQuery.Speed.one_handed) nextParams.set("event", "oh")
            else if (nextQuery.Fmc.enabled) nextParams.set("event", nextQuery.Fmc.computer_assisted ? "fmcca" : "fmc")
            else nextParams.delete("event")

            if (nextQuery.Speed.filters === undefined) nextParams.delete("filters")
                else nextParams.set("filters", String(nextQuery.Speed.filters))

            if (nextQuery.Speed.macros === undefined) nextParams.delete("macros")
                else nextParams.set("macros", String(nextQuery.Speed.macros))

            if (nextQuery.Speed.variant && nextQuery.Speed.variant !== "Default") nextParams.set("variant", nextQuery.Speed.variant)
                else nextParams.delete("variant")

            if (nextQuery.Speed.program && nextQuery.Speed.program !== "Default") nextParams.set("program", nextQuery.Speed.program)
                else nextParams.delete("program")
        }

        setSearchParams(nextParams)
    }

    // this is not part of category query
    function handleRecordHistoryChange(state: boolean) {
        setRecordHistory(state)
        if (state) {
            const nextParams = new URLSearchParams(searchParams.toString())
            nextParams.set("history", "true")
            setSearchParams(nextParams)
        } else {
            const nextParams = new URLSearchParams(searchParams.toString())
            nextParams.delete("history")
            setSearchParams(nextParams)
        }
    }


    function handleEventChange(event: string) {
        const nextQuery: CategoryQuery = {
            distinct: event === "distinct",
            Speed: {
                average: event === "avg",
                blind: event === "bld",
                filters: query.Speed.filters,
                macros: query.Speed.macros,
                one_handed: event === "oh",
                variant: query.Speed.variant,
                program: query.Speed.program,
            },
            Fmc: {
                enabled: event === "fmc" || event === "fmcca",
                computer_assisted: event === "fmcca",
            },
        }

        syncUrl(nextQuery)
    }

    function handleMacroChange(nextMacros: boolean|undefined) {
        const nextQuery: CategoryQuery = {
            ...query,
            Speed: {
            ...query.Speed,
            macros: nextMacros,
            },
        }

        syncUrl(nextQuery)
    }

    function handleFilterChange(nextFilters: boolean|undefined) {
        const nextQuery: CategoryQuery = {
            ...query,
            Speed: {
            ...query.Speed,
            filters: nextFilters,
            },
        }

        syncUrl(nextQuery)
    }

    function handleVariantChange(name: string) {
        const selectedOption = FIXED_VARIANT_OPTIONS.find((option) => option.label === name)
        if (!selectedOption) return

        const nextQuery: CategoryQuery = {
            ...query,
            Speed: {
                ...query.Speed,
                program: selectedOption.program,
                variant: selectedOption.variant,
            },
        }

        syncUrl(nextQuery)
    }

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
                                    <SelectItem value="single"><Timer/>Single speedsolve</SelectItem>
                                    <SelectItem value="avg"><Sigma/>Average speedsolve</SelectItem>
                                    <SelectItem value="bld"><EyeOff/>Blindfolded</SelectItem>
                                    <SelectItem value="oh"><Hand/>One-handed</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Fewest moves</SelectLabel>
                                    <SelectItem value="fmc"><ScrollText/>Fewest moves</SelectItem>
                                    <SelectItem value="fmcca"><Laptop/>Computer-assisted</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Aggregate</SelectLabel>
                                    <SelectItem value="distinct"><Shapes/>Distinct Puzzles</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </Field>

                    <Field className="w-min">
                        <FieldLabel>Piece filters allowed</FieldLabel>
                        <ButtonGroup>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={filters == undefined ? "default" : "secondary"} onClick={() => handleFilterChange(undefined)}>Default</Button>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={filters == false ? "default" : "secondary"} onClick={() => handleFilterChange(false)}>No</Button>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={filters == true ? "default" : "secondary"} onClick={() => handleFilterChange(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>


                    <Field className="w-min">
                        <FieldLabel>Macros allowed</FieldLabel>
                        <ButtonGroup>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={macros == undefined ? "default" : "secondary"} onClick={() => handleMacroChange(undefined)}>Default</Button>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={macros == false ? "default" : "secondary"} onClick={() => handleMacroChange(false)}>No</Button>
                            <Button disabled={selectedEvent == "distinct" || selectedEvent == "fmc" || selectedEvent == "fmcca"} variant={macros == true ? "default" : "secondary"} onClick={() => handleMacroChange(true)}>Yes</Button>
                        </ButtonGroup>
                    </Field>

                    {puzzleId !== undefined ?
                        <>
                            <Field className="w-min">
                                <FieldLabel>Variant</FieldLabel>
                                <ButtonGroup>
                                    {availableVariantOptions.map((variant) => (
                                        <Button
                                            key={variant.label}
                                            disabled={selectedEvent == "fmc" || selectedEvent == "fmcca"}
                                            variant={selectedVariant === variant.variant && selectedProgram === variant.program ? "default" : "secondary"}
                                            onClick={() => handleVariantChange(variant.label)}
                                        >
                                            {variant.label}
                                        </Button>
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
