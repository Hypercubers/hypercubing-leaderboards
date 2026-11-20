import { DatabaseX } from "lucide-react";
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "./ui/empty";

interface ErrorDataProps {
    word?: string
}

function ErrorData({word}: ErrorDataProps) {
    return (
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant="icon">
                <DatabaseX />
                </EmptyMedia>
                <EmptyTitle>Error</EmptyTitle>
                <EmptyDescription>Error loading {word ?? "data"}</EmptyDescription>
            </EmptyHeader>
        </Empty>
    )
}

export default ErrorData