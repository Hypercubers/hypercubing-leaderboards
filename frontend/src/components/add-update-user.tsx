import { CirclePlus, Pencil } from "lucide-react";
import { Button } from "./ui/button";
import { Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";

interface props {
    add: boolean,
    fillValues?: {
        id: number,
        name: string,
        email?: string,
        discord_id?: string,
        moderator_notes?: string,
        moderator: boolean,
        dummy: boolean
    }
}

function EditUser(props: props) {
    return (
        <Dialog>
            <DialogTrigger>
                <Button>{props.add ? <CirclePlus/> : <Pencil/>} {props.add ? "Add user" : "Edit user"}</Button>
            </DialogTrigger>
            <DialogContent>
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
            </DialogContent>
        </Dialog>
    )
}

export default EditUser