import { Ghost } from "lucide-react";
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "./ui/empty";

interface EmptyDataProps {
    word?: string
}

function EmptyData({word}: EmptyDataProps) {
    return (
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant="icon">
                <Ghost />
                </EmptyMedia>
                <EmptyTitle>No {word ?? "data"}</EmptyTitle>
                <EmptyDescription>No {word ?? "data"} found</EmptyDescription>
            </EmptyHeader>
        </Empty>
    )
}

export default EmptyData