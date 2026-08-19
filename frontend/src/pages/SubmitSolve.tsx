"use client"

import Header from "@/components/header"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList } from "@/components/ui/combobox"
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { getPrograms, getPuzzles, getVariants, submitSolve, type Program, type Puzzle, type Variant } from "@/lib/backend"
import { ChevronDownIcon } from "lucide-react"
import { useEffect, useState } from "react"

import { format } from "date-fns"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Checkbox } from "@/components/ui/checkbox"
import { InputGroup, InputGroupAddon, InputGroupInput, InputGroupText } from "@/components/ui/input-group"

import { useForm } from '@tanstack/react-form'
import * as z from "zod"
import { FileUpload } from "@/components/ui/file-upload"
import ProgramIcon from "@/components/icon/program-icon"
import { useAuth } from "@/lib/auth-context"
import { useNavigate } from "react-router-dom"



// Zod schema for the form
const solveDataSchema = z.object({
    solve_id: z.number().optional(),

    // Event
    puzzle_id: z.number().min(1, "Please select a puzzle"),
    variant_id: z.number().optional(),
    program_id: z.number().min(1, "Please select a program"),

    // Metadata
    solver_id: z.number().optional(),
    solve_date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "Please choose a date"),
    solver_notes: z.string().optional(),
    moderator_notes: z.string().optional(),

    // Speedsolve
    solve_h: z.number().min(0, "Please enter a positive number").max(999, "Please enter a number that is less than 1000").optional(),
    solve_m: z.number().min(0, "Please enter a positive number").max(59, "Please enter a number that is less than 60").optional(),
    solve_s: z.number().min(0, "Please enter a positive number").max(59, "Please enter a number that is less than 60").optional(),
    solve_cs: z.number().min(0, "Please enter a positive number").max(99, "Please enter a number that is less than 100").optional(),
    uses_filters: z.boolean(),
    uses_macros: z.boolean(),
    average: z.boolean(),
    one_handed: z.boolean(),
    blind: z.boolean(),
    memo_h: z.number().min(0, "Please enter a positive number").max(999, "Please enter a number that is less than 1000").optional(),
    memo_m: z.number().min(0, "Please enter a positive number").max(59, "Please enter a number that is less than 60").optional(),
    memo_s: z.number().min(0, "Please enter a positive number").max(59, "Please enter a number that is less than 60").optional(),
    memo_cs: z.number().min(0, "Please enter a positive number").max(99, "Please enter a number that is less than 100").optional(),
    video_url: z
    .union([
        z.string().url("Must be a valid URL"),
        z.literal(""),
    ])
    .optional()
    .transform((value) => value === "" ? undefined : value),

    // Fewest moves
    move_count: z.number().min(0).optional(),
    computer_assisted: z.boolean(),
    replace_log_file: z.boolean().optional(),
    log_file: z.instanceof(File).optional(),
})

type SolveData = z.infer<typeof solveDataSchema>


function SubmitSolve() {
    // gets current lists of puzzles/programs for dropdowns
    const [puzzles, setPuzzles] = useState<Puzzle[]>([])
    const [programs, setPrograms] = useState<Program[]>([])
    const [variants, setVariants] = useState<Variant[]>([])
    // used for the Calendar date picker
    const [date, setDate] = useState<Date|undefined>(undefined)
    // gets the logged in user for Solver ID
    const { user } = useAuth()
    const navigate = useNavigate()

    const handleDateSelect = (day?: Date) => {
        setDate(day)
        form.setFieldValue("solve_date", day ? format(day, "yyyy-MM-dd") : "")
    }

    // gets puzzles, programs, and variants on page load
    useEffect(() => {
        getPuzzles().then(setPuzzles)
        getPrograms().then(setPrograms)
        getVariants().then(setVariants)
    }, [])

    const form = useForm({
        defaultValues: {
            solve_id: 0,
            puzzle_id: 0,
            variant_id: undefined,
            program_id: 0,
            solver_id: user?.id,
            solve_date: '',
            solver_notes: '',
            moderator_notes: '',
            solve_h: undefined,
            solve_m: undefined,
            solve_s: undefined,
            solve_cs: undefined,
            uses_filters: true,
            uses_macros: false,
            average: false,
            one_handed: false,
            blind: false,
            memo_h: undefined,
            memo_m: undefined,
            memo_s: undefined,
            memo_cs: undefined,
            video_url: undefined,
            move_count: undefined,
            computer_assisted: false,
            replace_log_file: false,
            log_file: undefined,
            // audit_log_comment: ''
        } as SolveData,
        validators: {
            onSubmit: solveDataSchema
        },
        onSubmit: async ({value}) => {
            const response = await submitSolve(value)
            if (response) navigate(response.redirect)
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

                                <form.Field
                                    name="variant_id"
                                    children={(field) => {
                                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                        return (
                                            <Field>
                                                <FieldLabel htmlFor={field.name}>Variant</FieldLabel>

                                                <Select
                                                    name={field.name}
                                                    value={field.state.value != null ? field.state.value.toString() : "none"}
                                                    onValueChange={(value) => {
                                                        field.handleChange(value === "none" ? undefined: Number(value))
                                                    }}
                                                >
                                                    <SelectTrigger id={field.name}>
                                                        <SelectValue placeholder="Select a variant" />
                                                    </SelectTrigger>
                                                    <SelectContent position="popper">

                                                        <SelectItem key={0} aria-selected value="none">Default</SelectItem>
                                                        {variants.map((variant) => (
                                                            <SelectItem key={variant.id} value={variant.id.toString()}>
                                                                {variant.name}
                                                            </SelectItem>
                                                        ))}

                                                    </SelectContent>
                                                </Select>
                                                <FieldDescription>Visual representation + available moves</FieldDescription>
                                                {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                            </Field>

                                        )

                                    }}
                                />


                                <form.Field
                                name="program_id"
                                children={(field) => {
                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                    const selectedProgram = programs.find((p) => p.id == field.state.value) || null
                                    return (
                                        <Field>
                                            <FieldLabel htmlFor={field.name}>Computer program</FieldLabel>
                                                <Combobox
                                                    items={programs}
                                                    itemToStringLabel={(program: Program) => program.name}
                                                    value={selectedProgram}
                                                    onValueChange={(selectedItem: Program | null | undefined) => {
                                                    if (selectedItem) {
                                                        field.handleChange(selectedItem.id)
                                                    } else {
                                                        field.handleChange(0)
                                                    }
                                                }}
                                                >
                                                    <ComboboxInput
                                                        id={field.name}
                                                        name={field.name}
                                                        onBlur={field.handleBlur}
                                                        aria-invalid={isInvalid}
                                                        placeholder="Select a program"
                                                    />
                                                    <ComboboxContent>
                                                        <ComboboxEmpty>No programs found</ComboboxEmpty>
                                                        <ComboboxList>
                                                            {(item) => (
                                                                <ComboboxItem className="inline-flex items-center [&_svg]:size-[1.6rem]!" key={item.id} value={(item)}>
                                                                        <ProgramIcon abbr={item.abbr}/>
                                                                        &nbsp; {item.name}
                                                                </ComboboxItem>
                                                            )}
                                                        </ComboboxList>
                                                    </ComboboxContent>
                                                </Combobox>

                                            <FieldDescription>Select “N/A” for solves done without using a computer</FieldDescription>
                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                        </Field>

                                    )
                                }}
                                />

                            </FieldGroup>

                        </CardContent>

                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Speedsolve</CardTitle>
                        </CardHeader>
                            <CardContent>
                                <FieldSet>
                                    <FieldLegend>Solve Duration</FieldLegend>
                                </FieldSet>
                                <FieldGroup className="grid grid-cols-4 items-left mb-1 gap-x-1">
                                    <form.Field
                                        name="solve_h"
                                        children={(field) => {
                                            const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                            return (
                                                <Field>
                                                    <FieldLabel htmlFor={field.name}>Hours</FieldLabel>
                                                    <InputGroup>
                                                        <InputGroupInput
                                                        className="text-right"
                                                        aria-invalid={isInvalid}
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onChange={(e) => {
                                                            const val = e.target.value
                                                            if (Number(val)) {
                                                                field.handleChange(Number(val))
                                                            } else {
                                                                field.handleChange(0)
                                                            }
                                                        }}
                                                        />
                                                        <InputGroupAddon align="inline-end">
                                                            <InputGroupText>h</InputGroupText>
                                                        </InputGroupAddon>
                                                    </InputGroup>
                                                    {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="solve_m"
                                        children={(field) => {
                                            const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                            return (
                                                <Field>
                                                    <FieldLabel htmlFor={field.name}>Minutes</FieldLabel>
                                                    <InputGroup>
                                                        <InputGroupInput
                                                        className="text-right"
                                                        aria-invalid={isInvalid}
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onChange={(e) => {
                                                            const val = e.target.value
                                                            if (Number(val)) {
                                                                field.handleChange(Number(val))
                                                            } else {
                                                                field.handleChange(0)
                                                            }
                                                        }}
                                                        />
                                                        <InputGroupAddon align="inline-end">
                                                            <InputGroupText>m</InputGroupText>
                                                        </InputGroupAddon>
                                                    </InputGroup>
                                                    {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="solve_s"
                                        children={(field) => {
                                            const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                            return (
                                                <Field>
                                                    <FieldLabel htmlFor={field.name}>Seconds</FieldLabel>
                                                    <InputGroup>
                                                        <InputGroupInput
                                                        className="text-right"
                                                        aria-invalid={isInvalid}
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onChange={(e) => {
                                                            const val = e.target.value
                                                            if (Number(val)) {
                                                                field.handleChange(Number(val))
                                                            } else {
                                                                field.handleChange(0)
                                                            }
                                                        }}
                                                        />
                                                        <InputGroupAddon align="inline-end">
                                                            <InputGroupText>s</InputGroupText>
                                                        </InputGroupAddon>
                                                    </InputGroup>
                                                    {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="solve_cs"
                                        children={(field) => {
                                            const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                            return (
                                                <Field>
                                                    <FieldLabel htmlFor={field.name}>Centiseconds</FieldLabel>
                                                    <InputGroup>
                                                        <InputGroupInput
                                                        className="text-right"
                                                        aria-invalid={isInvalid}
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onChange={(e) => {
                                                            const val = e.target.value
                                                            if (Number(val)) {
                                                                field.handleChange(Number(val))
                                                            } else {
                                                                field.handleChange(0)
                                                            }
                                                        }}
                                                        />
                                                        <InputGroupAddon align="inline-end">
                                                            <InputGroupText>cs</InputGroupText>
                                                        </InputGroupAddon>
                                                    </InputGroup>
                                                    {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                </Field>
                                            )
                                        }}
                                    />

                                </FieldGroup>
                                <FieldDescription className="w-full">Truncate to 0.01 seconds</FieldDescription>



                                {/* Checkboxes */}



                                <FieldGroup className="gap-4 mt-6 mb-6" data-slot="checkbox-group">
                                    <form.Field
                                        name="uses_filters"
                                        children={(field) => {
                                            return (
                                                <Field orientation={"horizontal"}>
                                                    <Checkbox
                                                    id={field.name}
                                                    name={field.name}
                                                    checked={field.state.value}
                                                    onCheckedChange={(checked) => {
                                                        field.handleChange(checked === true)
                                                    }}
                                                    />
                                                    <FieldLabel htmlFor={field.name}>Uses filters</FieldLabel>
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="uses_macros"
                                        children={(field) => {
                                            return (
                                                <Field orientation={"horizontal"}>
                                                    <Checkbox
                                                    id={field.name}
                                                    name={field.name}
                                                    checked={field.state.value}
                                                    onCheckedChange={(checked) => {
                                                        field.handleChange(checked === true)
                                                    }}
                                                    />
                                                    <FieldLabel htmlFor={field.name}>Uses macros</FieldLabel>
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="average"
                                        children={(field) => {
                                            return (
                                                <Field orientation={"horizontal"}>
                                                    <Checkbox
                                                    id={field.name}
                                                    name={field.name}
                                                    checked={field.state.value}
                                                    onCheckedChange={(checked) => {
                                                        field.handleChange(checked === true)
                                                    }}
                                                    />
                                                    <FieldLabel htmlFor={field.name}>Average of 5</FieldLabel>
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="one_handed"
                                        children={(field) => {
                                            return (
                                                <Field orientation={"horizontal"}>
                                                    <Checkbox
                                                    id={field.name}
                                                    name={field.name}
                                                    checked={field.state.value}
                                                    onCheckedChange={(checked) => {
                                                        field.handleChange(checked === true)
                                                    }}
                                                    />
                                                    <FieldLabel htmlFor={field.name}>One-handed</FieldLabel>
                                                </Field>
                                            )
                                        }}
                                    />

                                    <form.Field
                                        name="blind"
                                        children={(field) => {
                                            return (
                                                <Field orientation={"horizontal"}>
                                                    <Checkbox
                                                    id={field.name}
                                                    name={field.name}
                                                    checked={field.state.value}
                                                    onCheckedChange={(checked) => {
                                                        field.handleChange(checked === true)
                                                    }}
                                                    />
                                                    <FieldLabel htmlFor={field.name}>Blindfolded</FieldLabel>
                                                </Field>
                                            )
                                        }}
                                    />

                                </FieldGroup>



                                {/* Blindfolded time input */}


                                <FieldSet>
                                    <FieldLegend>Memorization time (only for blindfolded solves)</FieldLegend>
                                </FieldSet>

                                <form.Subscribe selector={(state) => state.values.blind}>
                                    {(blind) => (
                                        <fieldset disabled={!blind}>
                                            <FieldGroup className="grid grid-cols-4 items-left mb-1 gap-x-1">
                                            <form.Field
                                                name="memo_h"
                                                children={(field) => {
                                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                                    return (
                                                        <Field>
                                                            <FieldLabel htmlFor={field.name}>Hours</FieldLabel>
                                                            <InputGroup>
                                                                <InputGroupInput
                                                                className="text-right"
                                                                aria-invalid={isInvalid}
                                                                id={field.name}
                                                                name={field.name}
                                                                value={field.state.value}
                                                                onChange={(e) => {
                                                                    const val = e.target.value
                                                                    if (Number(val)) {
                                                                        field.handleChange(Number(val))
                                                                    } else {
                                                                        field.handleChange(0)
                                                                    }
                                                                }}
                                                                />
                                                                <InputGroupAddon align="inline-end">
                                                                    <InputGroupText>h</InputGroupText>
                                                                </InputGroupAddon>
                                                            </InputGroup>
                                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                        </Field>
                                                    )
                                                }}
                                            />

                                            <form.Field
                                                name="memo_m"
                                                children={(field) => {
                                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                                    return (
                                                        <Field>
                                                            <FieldLabel htmlFor={field.name}>Minutes</FieldLabel>
                                                            <InputGroup>
                                                                <InputGroupInput
                                                                className="text-right"
                                                                aria-invalid={isInvalid}
                                                                id={field.name}
                                                                name={field.name}
                                                                value={field.state.value}
                                                                onChange={(e) => {
                                                                    const val = e.target.value
                                                                    if (Number(val)) {
                                                                        field.handleChange(Number(val))
                                                                    } else {
                                                                        field.handleChange(0)
                                                                    }
                                                                }}
                                                                />
                                                                <InputGroupAddon align="inline-end">
                                                                    <InputGroupText>m</InputGroupText>
                                                                </InputGroupAddon>
                                                            </InputGroup>
                                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                        </Field>
                                                    )
                                                }}
                                            />

                                            <form.Field
                                                name="memo_s"
                                                children={(field) => {
                                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                                    return (
                                                        <Field>
                                                            <FieldLabel htmlFor={field.name}>Seconds</FieldLabel>
                                                            <InputGroup>
                                                                <InputGroupInput
                                                                className="text-right"
                                                                aria-invalid={isInvalid}
                                                                id={field.name}
                                                                name={field.name}
                                                                value={field.state.value}
                                                                onChange={(e) => {
                                                                    const val = e.target.value
                                                                    if (Number(val)) {
                                                                        field.handleChange(Number(val))
                                                                    } else {
                                                                        field.handleChange(0)
                                                                    }
                                                                }}
                                                                />
                                                                <InputGroupAddon align="inline-end">
                                                                    <InputGroupText>s</InputGroupText>
                                                                </InputGroupAddon>
                                                            </InputGroup>
                                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                        </Field>
                                                    )
                                                }}
                                            />

                                            <form.Field
                                                name="memo_cs"
                                                children={(field) => {
                                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                                    return (
                                                        <Field>
                                                            <FieldLabel htmlFor={field.name}>Centiseconds</FieldLabel>
                                                            <InputGroup>
                                                                <InputGroupInput
                                                                className="text-right"
                                                                aria-invalid={isInvalid}
                                                                id={field.name}
                                                                name={field.name}
                                                                value={field.state.value}
                                                                onChange={(e) => {
                                                                    const val = e.target.value
                                                                    if (Number(val)) {
                                                                        field.handleChange(Number(val))
                                                                    } else {
                                                                        field.handleChange(0)
                                                                    }
                                                                }}
                                                                />
                                                                <InputGroupAddon align="inline-end">
                                                                    <InputGroupText>cs</InputGroupText>
                                                                </InputGroupAddon>
                                                            </InputGroup>
                                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                                        </Field>
                                                    )
                                                }}
                                            />

                                        </FieldGroup>
                                        <FieldDescription className="w-full">Truncate to 0.01 seconds</FieldDescription>
                                        </fieldset>
                                    )}
                                </form.Subscribe>

                                <form.Field
                                name="video_url"
                                children={(field) => {
                                    return (
                                        <Field className="mt-4">
                                            <FieldLabel htmlFor={field.name}>Video link</FieldLabel>
                                            <Input
                                            type="text"
                                            placeholder="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            >
                                            </Input>
                                            <FieldDescription>Required for speedsolves</FieldDescription>
                                        </Field>

                                    )
                                }}
                                />

                            </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Metadata</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <FieldGroup>
                                <form.Field
                                name="solve_date"
                                children={(field) => {
                                    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                                    return (
                                        <Field>
                                            <FieldLabel htmlFor={field.name}>Solve date</FieldLabel>
                                            <Popover>
                                                <PopoverTrigger asChild aria-invalid={isInvalid}>
                                                    <Button
                                                    variant="outline"
                                                    data-empty={!date}
                                                    className="bg-accent w-53 justify-between text-left font-normal data-[empty=true]:text-muted-foreground"
                                                    >
                                                    {date ? format(date, "yyyy-MM-dd") : "Pick a date"}
                                                    <ChevronDownIcon />
                                                    </Button>
                                                </PopoverTrigger>
                                                <PopoverContent className="w-auto p-0" align="start">
                                                    <Calendar
                                                        mode="single"
                                                        selected={date}
                                                        onSelect={handleDateSelect}
                                                        defaultMonth={date}
                                                    />
                                                </PopoverContent>
                                            </Popover>
                                            {isInvalid && <FieldError errors={field.state.meta.errors}/>}
                                        </Field>

                                    )
                                }}

                                />

                                <form.Field
                                name="solver_notes"
                                children={(field) => {
                                    return (
                                        <Field>
                                            <FieldLabel htmlFor="notes">Notes</FieldLabel>
                                            <Textarea
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onChange={(e) => field.handleChange(e.target.value)}
                                            />
                                            <FieldDescription>
                                                <div>
                                                    <p>For average-of-5 events, please list all 5 single-solve times. </p>
                                                    <p>If you selected “Other” for puzzle, variant, or program, explain here. </p>
                                                    <p>Material non-physical puzzles (e.g., hemimegaminx) should use “Default” variant and “N/A” computer program. </p>
                                                </div>
                                            </FieldDescription>
                                        </Field>
                                    )
                                }}

                                />

                            </FieldGroup>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl">Fewest moves</CardTitle>
                        </CardHeader>
                        <CardContent className="flex flex-col gap-4">
                            <FieldGroup>
                                <form.Field
                                name="move_count"
                                children={(field) => {
                                    return (
                                        <Field>
                                            <FieldLabel>Move count (STM)</FieldLabel>
                                            <Input
                                            type="number"
                                            className="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onChange={(e) => {
                                                const val = e.target.value
                                                if (Number(val)) {
                                                    field.handleChange(Number(val))
                                                } else {
                                                    field.handleChange(0)
                                                }
                                            }}
                                            ></Input>
                                        </Field>
                                    )
                                }}

                                />


                                <form.Field
                                name="computer_assisted"
                                children={(field) => {
                                    return (
                                        <Field orientation={"horizontal"}>
                                            <Checkbox
                                            id={field.name}
                                            name={field.name}
                                            checked={field.state.value}
                                            onCheckedChange={(checked) => {
                                                field.handleChange(checked === true)
                                            }}
                                            />
                                            <FieldLabel htmlFor={field.name}>Computer assisted</FieldLabel>
                                        </Field>
                                    )
                                }}
                                />

                                <form.Field
                                name="log_file"
                                children={(field) => {
                                    return (
                                        <Field>
                                            <FieldLabel htmlFor="logfile">Log file</FieldLabel>
                                            <FileUpload
                                                accept=".log,.hsc"
                                                maxFiles={1}
                                                multiple={false}
                                                onFilesChange={(files) => {
                                                    const file = files[0]?.file
                                                    field.handleChange(file instanceof File ? file : undefined)
                                                }}
                                            />
                                        </Field>
                                    )
                                }}
                                />
                            </FieldGroup>
                        </CardContent>
                    </Card>
                </div>
                <div className="flex mt-4 w-full">
                    <Button className="w-1/2" type="submit">Submit solve</Button>
                </div>


                <div>
                    <form.Subscribe selector={(state) => [state.canSubmit, state.values]}>
                        {(values) => (
                            <pre>{JSON.stringify(values, null, 2)}</pre>
                        )}
                    </form.Subscribe>
                </div>

            </form>

        </>
    )
}

export default SubmitSolve
