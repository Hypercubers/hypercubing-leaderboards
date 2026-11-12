import { useForm } from "@tanstack/react-form"
import * as z from "zod"
import { useRef } from "react"
import { updateVariant, type Variant } from "@/lib/backend"
import { DialogClose, DialogFooter, DialogHeader, DialogTitle } from "./ui/dialog"
import { Field, FieldError, FieldGroup, FieldLabel } from "./ui/field"
import { Input } from "./ui/input"
import { Button } from "./ui/button"
import { Checkbox } from "./ui/checkbox"
import { Textarea } from "./ui/textarea"


interface EditVariantProps {
    add: boolean
    fillValues?: Variant
}

const variantFormSchema = z.object({
    id: z.number().int().nonnegative().optional(),
    name: z.string().trim().min(1, "Name is required"),
    prefix: z.string(),
    suffix: z.string(),
    abbr: z.string().trim().min(1, "Abbreviation is required"),
    material_by_default: z.boolean(),
    primary_filters: z.boolean(),
    primary_macros: z.boolean(),
    audit_log_comment: z.string(),
})

function EditVariant({ add, fillValues }: EditVariantProps) {
    const closeButtonRef = useRef<HTMLButtonElement>(null)
    const form = useForm({
        defaultValues: {
            id: fillValues?.id,
            name: fillValues?.name ?? "",
            prefix: fillValues?.prefix ?? "",
            suffix: fillValues?.suffix ?? "",
            abbr: fillValues?.abbr ?? "",
            material_by_default: fillValues?.material_by_default ?? false,
            primary_filters: fillValues?.primary_filters ?? false,
            primary_macros: fillValues?.primary_macros ?? false,
            audit_log_comment: "",
        },
        validators: {
            onSubmit: ({ value }) => {
                const result = variantFormSchema.safeParse(value)
                return result.success ? undefined : result.error.message
            },
        },
        onSubmit: async ({ value }) => {
            await updateVariant(value)
            closeButtonRef.current?.click()
            window.location.reload()
        },
    })


    return (
        <form
            onSubmit={(event) => {
                event.preventDefault()
                event.stopPropagation()
                form.handleSubmit()
            }}
        >
            <DialogHeader>
                <DialogTitle>{add ? "Add" : "Edit"} variant</DialogTitle>
            </DialogHeader>

            <FieldGroup className="mt-6">
                <form.Field
                    name="id"
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>ID</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(event) => field.handleChange(event.target.value === "" ? undefined : Number(event.target.value))}
                                    onBlur={field.handleBlur}
                                    aria-invalid={isInvalid}
                                />
                                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                            </Field>
                        )
                    }}
                />
                <form.Field
                    name="name"
                    validators={{
                        onSubmit: ({ value }) => value.trim().length > 0 ? undefined : { message: "Name is required" },
                    }}
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Name</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(event) => field.handleChange(event.target.value)}
                                    onBlur={field.handleBlur}
                                    aria-invalid={isInvalid}
                                />
                                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                            </Field>
                        )
                    }}
                />
                <form.Field
                    name="prefix"
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Prefix</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(event) => field.handleChange(event.target.value)}
                                    onBlur={field.handleBlur}
                                    aria-invalid={isInvalid}
                                />
                                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                            </Field>
                        )
                    }}
                />
                <form.Field
                    name="suffix"
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Suffix</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(event) => field.handleChange(event.target.value)}
                                    onBlur={field.handleBlur}
                                    aria-invalid={isInvalid}
                                />
                                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                            </Field>
                        )
                    }}
                />
                <form.Field
                    name="abbr"
                    validators={{
                        onSubmit: ({ value }) => value.trim().length > 0 ? undefined : { message: "Abbreviation is required" },
                    }}
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Abbreviation</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(event) => field.handleChange(event.target.value)}
                                    onBlur={field.handleBlur}
                                    aria-invalid={isInvalid}
                                />
                                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                            </Field>
                        )
                    }}
                />
                <form.Field
                    name="material_by_default"
                    children={(field) => (
                        <Field orientation="horizontal">
                            <Checkbox
                                id={field.name}
                                name={field.name}
                                checked={field.state.value}
                                onCheckedChange={(checked) => field.handleChange(checked === true)}
                            />
                            <FieldLabel htmlFor={field.name}>Material</FieldLabel>
                        </Field>
                    )}
                />
                <form.Field
                    name="primary_filters"
                    children={(field) => (
                        <Field orientation="horizontal">
                            <Checkbox
                                id={field.name}
                                name={field.name}
                                checked={field.state.value}
                                onCheckedChange={(checked) => field.handleChange(checked === true)}
                            />
                            <FieldLabel htmlFor={field.name}>Allow filters</FieldLabel>
                        </Field>
                    )}
                />
                <form.Field
                    name="primary_macros"
                    children={(field) => (
                        <Field orientation="horizontal">
                            <Checkbox
                                id={field.name}
                                name={field.name}
                                checked={field.state.value}
                                onCheckedChange={(checked) => field.handleChange(checked === true)}
                            />
                            <FieldLabel htmlFor={field.name}>Allow macros</FieldLabel>
                        </Field>
                    )}
                />
                <form.Field
                    name="audit_log_comment"
                    children={(field) => (
                        <Field>
                            <FieldLabel htmlFor={field.name}>What you changed and why (optional)</FieldLabel>
                            <Textarea
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                onChange={(event) => field.handleChange(event.target.value)}
                                onBlur={field.handleBlur}
                            />
                        </Field>
                    )}
                />  
            </FieldGroup>

            <DialogFooter className="mt-6">
                <DialogClose asChild>
                    <button ref={closeButtonRef} type="button" hidden />
                </DialogClose>
                <DialogClose asChild>
                    <Button type="button" variant="outline">Cancel</Button>
                </DialogClose>
                <Button type="submit">{add ? "Add variant" : "Save changes"}</Button>
            </DialogFooter>
        </form>
    )
}

export default EditVariant