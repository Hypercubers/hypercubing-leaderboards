import { Button } from "./ui/button";
import { Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import type { User } from "@/lib/backend";
import { Field, FieldGroup, FieldLabel } from "./ui/field";
import { Input } from "./ui/input";
import { Textarea } from "./ui/textarea";
import { Checkbox } from "./ui/checkbox";

interface props {
    add: boolean,
    fillValues?: User
}

function EditUser(props: props) {
    return (
        <>
            <DialogHeader>
                <DialogTitle>{props.add ? "Add" : "Edit"} user</DialogTitle>
            </DialogHeader>
            <FieldGroup>
                

                <Field>
                    <FieldLabel>
                        Name (optional)
                    </FieldLabel>
                    <Input></Input>
                </Field>
                <Field>
                    <FieldLabel>
                        Email (optional)
                    </FieldLabel>
                    <Input></Input>
                </Field>
                <Field>
                    <FieldLabel>
                        Discord ID (optional)
                    </FieldLabel>
                    <Input></Input>
                </Field>
                <Field>
                    <FieldLabel>
                        Moderator notes (optional)
                    </FieldLabel>
                    <Textarea></Textarea>
                </Field>
                <Field orientation="horizontal">
                    <FieldLabel>
                        Is moderator?
                    </FieldLabel>
                    <Checkbox></Checkbox>
                </Field>
                <Field orientation="horizontal">
                    <FieldLabel>
                        Is dummy?
                    </FieldLabel>
                    <Checkbox></Checkbox>
                </Field>
                <Field>
                    <FieldLabel>
                        What you changed and why (optional)
                    </FieldLabel>
                    <Textarea></Textarea>
                </Field>
                
            </FieldGroup>
            <DialogFooter>
                <DialogClose asChild>
                    <Button variant="outline">Cancel</Button>
                </DialogClose>
                <Button>Save changes</Button>
            </DialogFooter>
        </>
    )
}

export default EditUser