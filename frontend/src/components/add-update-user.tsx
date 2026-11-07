import { useForm } from "@tanstack/react-form"
import * as z from "zod"
import { useRef } from "react"

import { updateUser, type User } from "@/lib/backend"
import { Button } from "./ui/button"
import { Checkbox } from "./ui/checkbox"
import { DialogClose, DialogFooter, DialogHeader, DialogTitle } from "./ui/dialog"
import { Field, FieldError, FieldGroup, FieldLabel } from "./ui/field"
import { Input } from "./ui/input"
import { Textarea } from "./ui/textarea"

interface EditUserProps {
    add: boolean
    fillValues?: User
}

const userFormSchema = z.object({
    id: z.number().int().nonnegative().optional(),
    name: z.string(),
    email: z.string().email().or(z.literal("")),
    discord: z.number().int().nonnegative().optional(),
    moderator: z.boolean(),
    moderator_notes: z.string(),
    dummy: z.boolean(),
    audit_log_comment: z.string(),
})

function EditUser({ add, fillValues }: EditUserProps) {
    const closeButtonRef = useRef<HTMLButtonElement>(null)
    const form = useForm({
        defaultValues: {
            id: fillValues?.id,
            name: fillValues?.name ?? "",
            email: fillValues?.email ?? "",
            discord: fillValues?.discord_id ?? undefined,
            moderator: fillValues?.moderator ?? false,
            moderator_notes: fillValues?.moderator_notes ?? "",
            dummy: fillValues?.dummy ?? false,
            audit_log_comment: "",
        },
        validators: {
            onSubmit: ({ value }) => {
                const result = userFormSchema.safeParse(value)
                return result.success ? undefined : result.error.message
            },
        },
        onSubmit: async ({ value }) => {
            await updateUser(value)
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
                <DialogTitle>{add ? "Add" : "Edit"} user</DialogTitle>
            </DialogHeader>

            <FieldGroup className="mt-6">
                <form.Field
                    name="id"
                    children={(field) => (
                        <Field>
                            <FieldLabel htmlFor={field.name}>ID (leave blank to assign automatically)</FieldLabel>
                            <Input
                                id={field.name}
                                name={field.name}
                                type="number"
                                min={0}
                                step={1}
                                value={field.state.value ?? ""}
                                onChange={(event) => {
                                    const value = event.target.value
                                    field.handleChange(value === "" ? undefined : Number(value))
                                }}
                                onBlur={field.handleBlur}
                            />
                        </Field>
                    )}
                />

                <form.Field
                    name="name"
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Name (optional)</FieldLabel>
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
                    name="email"
                    children={(field) => {
                        const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
                        return (
                            <Field>
                                <FieldLabel htmlFor={field.name}>Email (optional)</FieldLabel>
                                <Input
                                    id={field.name}
                                    name={field.name}
                                    type="email"
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
                    name="discord"
                    children={(field) => (
                        <Field>
                            <FieldLabel htmlFor={field.name}>Discord ID (optional)</FieldLabel>
                            <Input
                                id={field.name}
                                name={field.name}
                                type="number"
                                min={0}
                                step={1}
                                value={field.state.value ?? ""}
                                onChange={(event) => {
                                    const value = event.target.value
                                    field.handleChange(value === "" ? undefined : Number(value))
                                }}
                                onBlur={field.handleBlur}
                            />
                        </Field>
                    )}
                />

                <form.Field
                    name="moderator_notes"
                    children={(field) => (
                        <Field>
                            <FieldLabel htmlFor={field.name}>Moderator notes (optional)</FieldLabel>
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

                <FieldGroup data-slot="checkbox-group" className="gap-3">
                    <form.Field
                        name="moderator"
                        children={(field) => (
                            <Field orientation="horizontal">
                                <Checkbox
                                    id={field.name}
                                    name={field.name}
                                    checked={field.state.value}
                                    onCheckedChange={(checked) => field.handleChange(checked === true)}
                                />
                                <FieldLabel htmlFor={field.name}>Moderator</FieldLabel>
                            </Field>
                        )}
                    />
                    <form.Field
                        name="dummy"
                        children={(field) => (
                            <Field orientation="horizontal">
                                <Checkbox
                                    id={field.name}
                                    name={field.name}
                                    checked={field.state.value}
                                    onCheckedChange={(checked) => field.handleChange(checked === true)}
                                />
                                <FieldLabel htmlFor={field.name}>Dummy user</FieldLabel>
                            </Field>
                        )}
                    />
                </FieldGroup>

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
                <Button type="submit">{add ? "Add user" : "Save changes"}</Button>
            </DialogFooter>
        </form>
    )
}

export default EditUser
