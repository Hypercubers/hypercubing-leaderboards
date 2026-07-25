"use client"

import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList } from "@/components/ui/combobox"
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "@/components/ui/select"
import { getPrograms, getPuzzles, type Program, type Puzzle } from "@/lib/backend"
import { ChevronDownIcon } from "lucide-react"
import { useEffect, useState } from "react"

import { format } from "date-fns"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Checkbox } from "@/components/ui/checkbox"
import { Label } from "@/components/ui/label"
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group"

import { Temporal } from '@js-temporal/polyfill'

import { useForm } from '@tanstack/react-form'
import * as z from "zod"


// type FormFields = {
//     solve_id?: number,

//     // Event
//     puzzle_id: number
//     variant_id?: number,
//     program_id: number,

//     // Metadata
//     solver_id?: number,
//     solve_date: Temporal.PlainDate
//     solver_notes?: string,
//     moderator_notes?: string,

//     //Speedsolve
//     solve_h?: number,
//     solve_m?: number,
//     solve_s?: number,
//     solve_cs?: number,
//     uses_filters: boolean,
//     uses_macros: boolean,
//     average: boolean,
//     one_handed: boolean,
//     blind: boolean,
//     memo_h?: number,
//     memo_m?: number,
//     memo_s?: number,
//     memo_cs?: number,
//     video_url?: string,

//     //Fewest moves
//     move_count?: number,
//     computer_assisted: boolean,
//     replace_log_file?: boolean,
//     log_file?: Uint8Array

//     audit_log_comment?: string,
// }


// Create the Zod schema
export const solveDataSchema = z.object({
    solve_id: z.number().optional(),

    // Event
    puzzle_id: z.number().min(1, "Please select a puzzle"),
    variant_id: z.number().optional(),
    program_id: z.number().min(1, "Please select a program"),

    // Metadata
    solver_id: z.number().optional(),
    // TODO: make sure date data type works correctly
    solve_date: z.any(),
    solver_notes: z.string().optional(),
    moderator_notes: z.string().optional(),

    // Speedsolve
    solve_h: z.number().min(0).optional(),
    solve_m: z.number().min(0).max(59).optional(),
    solve_s: z.number().min(0).max(59).optional(),
    solve_cs: z.number().min(0).max(99).optional(),
    uses_filters: z.boolean().default(false),
    uses_macros: z.boolean().default(false),
    average: z.boolean().default(false),
    one_handed: z.boolean().default(false),
    blind: z.boolean().default(false),
    memo_h: z.number().min(0).optional(),
    memo_m: z.number().min(0).max(59).optional(),
    memo_s: z.number().min(0).max(59).optional(),
    memo_cs: z.number().min(0).max(99).optional(),
    video_url: z.url("Must be a valid URL").optional(),

    // Fewest moves
    move_count: z.number().min(0).optional(),
    computer_assisted: z.boolean().default(false),
    replace_log_file: z.boolean().optional(),
    log_file: z.instanceof(File).optional(),
})

type SolveData = z.infer<typeof solveDataSchema>


function SubmitSolve() {
    // gets current lists of puzzles/programs for dropdowns
    const [puzzles, setPuzzles] = useState<Puzzle[]>([])
    const [programs, setPrograms] = useState<Program[]>([])
    // used for the Calendar date picker
    const [date, setDate] = useState<Date>()

    // gets puzzles and programs on page load
    useEffect(() => {
        getPuzzles().then(setPuzzles)
        getPrograms().then(setPrograms)
    }, [])

    const form = useForm({
        defaultValues: {
            solve_id: 0,
            puzzle_id: 0,
            variant_id: 0,
            program_id: 0,
            solver_id: 0,
            solve_date: '',
            solver_notes: '',
            moderator_notes: '',
            solve_h: 0,
            solve_m: 0,
            solve_s: 0,
            solve_cs: 0,
            uses_filters: true,
            uses_macros: false,
            average: false,
            one_handed: false,
            blind: false,
            memo_h: 0,
            memo_m: 0,
            memo_s: 0,
            memo_cs: 0,
            video_url: '',
            move_count: 0,
            computer_assisted: false,
            replace_log_file: false,
            log_file: new File([""], "file"),
            audit_log_comment: ''
        } as SolveData,
        onSubmit: async ({value}) => {
            console.log("Form submitted", value)
        }
    })



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

            <form className="pb-4" onSubmit={(e) => {
                e.preventDefault()
                e.stopPropagation()
                form.handleSubmit()
            }}>
                <div className="grid md:grid-cols-2 mt-4 gap-4">
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl" >Event</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <FieldGroup>
                                <form.Field
                                name="puzzle_id"
                                children={(field) => {
                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                    const selectedPuzzle = puzzles.find((p) => p.id == field.state.value) || null
                                    return (
                                        <Field>
                                            <FieldLabel htmlFor={field.name}>Puzzle</FieldLabel>
                                            <Combobox
                                                items={puzzles}
                                                itemToStringLabel={(puzzle: Puzzle) => puzzle.name}
                                                value={selectedPuzzle}
                                                onValueChange={(selectedItem: Puzzle | null | undefined) => {
                                                    if (selectedItem) {
                                                        field.handleChange(selectedItem.id)
                                                    } else {
                                                        field.handleChange(0)
                                                    }
                                                }}
                                                filter={(item: Puzzle, query, itemToString) => {
                                                    const normalize = (value: string) =>
                                                        value.replaceAll("×", "x").toLowerCase()
                                                    const label = itemToString?.(item) ?? item.name
                                                    return normalize(label).includes(normalize(query))
                                                }}
                                            >
                                                <ComboboxInput
                                                    id={field.name}
                                                    name={field.name}
                                                    onBlur={field.handleBlur}
                                                    aria-invalid={isInvalid}
                                                    placeholder="Select a puzzle"

                                                />
                                                <ComboboxContent>
                                                    <ComboboxEmpty>No puzzles found</ComboboxEmpty>
                                                    <ComboboxList>
                                                        {(item) => (
                                                            <ComboboxItem key={item.id} value={item}>
                                                                {item.name}
                                                            </ComboboxItem>
                                                        )}
                                                    </ComboboxList>
                                                </ComboboxContent>
                                            </Combobox>

                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                        </Field>

                                    )
                                }}
                                />


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
                                        <Combobox items={programs}>
                                            <ComboboxInput placeholder="Select a program" />
                                            <ComboboxContent>
                                                <ComboboxEmpty>No programs found</ComboboxEmpty>
                                                <ComboboxList>
                                                    {(item) => (
                                                        <ComboboxItem key={item.id} value={(item.name)}>{(item.name)}</ComboboxItem>
                                                    )}
                                                </ComboboxList>
                                            </ComboboxContent>
                                        </Combobox>

                                    <FieldDescription>Select “N/A” for solves done without using a computer</FieldDescription>
                                    {/* {errors.program_id && <FieldError>{errors.program_id.message}</FieldError>} */}
                                </Field>
                            </FieldGroup>

                        </CardContent>

                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Speedsolve</CardTitle>
                            <CardContent>
                                <FieldGroup className="gap-0">
                                    {/* <FieldLegend>Solve duration</FieldLegend> */}
                                    <div className="grid grid-cols-4 items-left mb-1 gap-x-1">
                                        <Field>
                                        <InputGroup>
                                            <InputGroupInput type="text" />
                                                <InputGroupAddon align="inline-end">h</InputGroupAddon>
                                            </InputGroup>
                                        </Field>

                                        <InputGroup>
                                            <InputGroupInput type="text" />
                                            <InputGroupAddon align="inline-end">m</InputGroupAddon>
                                        </InputGroup>

                                        <InputGroup>
                                            <InputGroupInput type="text" />
                                            <InputGroupAddon align="inline-end">s</InputGroupAddon>
                                        </InputGroup>

                                        <InputGroup>
                                            <InputGroupInput type="text" />
                                            <InputGroupAddon align="inline-end">cs</InputGroupAddon>
                                        </InputGroup>
                                    </div>
                                </FieldGroup>
                                <FieldDescription className="w-full">Truncate to 0.01 seconds</FieldDescription>

                                <FieldGroup className="gap-4 mt-4">
                                    <Field orientation={"horizontal"}>
                                        <Checkbox checked></Checkbox>
                                        <Label>Uses filters</Label>
                                    </Field>
                                    <Field orientation={"horizontal"}>
                                        <Checkbox></Checkbox>
                                        <Label>Uses macros</Label>
                                    </Field>
                                    <Field orientation={"horizontal"}>
                                        <Checkbox></Checkbox>
                                        <Label>Average of 5</Label>
                                    </Field>
                                    <Field orientation={"horizontal"}>
                                        <Checkbox></Checkbox>
                                        <Label>One-handed</Label>
                                    </Field>
                                    <Field orientation={"horizontal"}>
                                        <Checkbox></Checkbox>
                                        <Label>Blindfolded</Label>
                                    </Field>

                                    <FieldGroup className="gap-0">
                                        <div className="grid grid-cols-4 items-left mb-1 gap-x-1">
                                            <Field>
                                            <InputGroup>
                                                <InputGroupInput type="text" />
                                                    <InputGroupAddon align="inline-end">h</InputGroupAddon>
                                                </InputGroup>
                                            </Field>

                                            <InputGroup>
                                                <InputGroupInput type="text" />
                                                <InputGroupAddon align="inline-end">m</InputGroupAddon>
                                            </InputGroup>

                                            <InputGroup>
                                                <InputGroupInput type="text" />
                                                <InputGroupAddon align="inline-end">s</InputGroupAddon>
                                            </InputGroup>

                                            <InputGroup>
                                                <InputGroupInput type="text" />
                                                <InputGroupAddon align="inline-end">cs</InputGroupAddon>
                                            </InputGroup>
                                        </div>
                                    </FieldGroup>
                                <FieldDescription className="w-full">Truncate to 0.01 seconds</FieldDescription>

                                <Field>
                                    <FieldLabel>Video link</FieldLabel>
                                    <Input type="text" placeholder="https://www.youtube.com/watch?v=dQw4w9WgXcQ"></Input>
                                    <FieldDescription>Required for speedsolves</FieldDescription>
                                </Field>

                                </FieldGroup>
                            </CardContent>
                        </CardHeader>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Metadata</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <FieldGroup>
                                <Field>
                                    <FieldLabel htmlFor="date">Solve date</FieldLabel>
                                    <Popover>
                                        <PopoverTrigger asChild>
                                            <Button
                                            variant="outline"
                                            data-empty={!date}
                                            className="bg-accent w-[212px] justify-between text-left font-normal data-[empty=true]:text-muted-foreground"
                                            >
                                            {date ? format(date, "yyyy-MM-dd") : <span>Pick a date</span>}
                                            <ChevronDownIcon />
                                            </Button>
                                        </PopoverTrigger>
                                        <PopoverContent className="w-auto p-0" align="start">
                                            <Calendar
                                            mode="single"
                                            selected={date}
                                            onSelect={setDate}
                                            defaultMonth={date}
                                            />
                                        </PopoverContent>
                                    </Popover>
                                    {/* {errors.solve_date && <FieldError>{errors.solve_date.message}</FieldError>} */}
                                </Field>

                                <Field>
                                    <FieldLabel htmlFor="notes">Notes</FieldLabel>
                                    <Textarea></Textarea>
                                    <FieldDescription>
                                        <div>
                                            <p>For average-of-5 events, please list all 5 single-solve times. </p>
                                            <p>If you selected “Other” for puzzle, variant, or program, explain here. </p>
                                            <p>Material non-physical puzzles (e.g., hemimegaminx) should use “Default” variant and “N/A” computer program. </p>
                                        </div>
                                    </FieldDescription>
                                </Field>
                            </FieldGroup>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Fewest moves</CardTitle>
                        </CardHeader>
                        <CardContent className="flex flex-col gap-4">
                             <Field>
                                <FieldLabel>Move count (STM)</FieldLabel>
                                <Input type="number" className="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"></Input>
                            </Field>

                            <Field orientation={"horizontal"}>
                                <Checkbox></Checkbox>
                                <Label>Computer assisted</Label>
                            </Field>

                            <Field>
                                <FieldLabel htmlFor="picture">Log file</FieldLabel>
                                <Input id="picture" type="file" />
                                <FieldDescription>Required for fewest-move solves</FieldDescription>
                            </Field>
                        </CardContent>
                    </Card>
                </div>
                <div className="flex mt-4 w-full">
                    <Button className="w-1/2" type="submit">Submit solve</Button>
                </div>


            </form>
        </>
    )
}

export default SubmitSolve
