import { Button } from "./ui/button";
import { Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import type { User } from "@/lib/backend";

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
            <p>test</p>
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