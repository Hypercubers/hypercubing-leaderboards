import { Spinner } from "./ui/spinner"
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "./ui/empty";


function Loading() {
    return (
         <Empty>
            <EmptyHeader>
                <EmptyMedia variant="default">
                    <Spinner className="size-6"/>
                </EmptyMedia>
                <EmptyTitle>Loading</EmptyTitle>
                <EmptyDescription>Fetching data...</EmptyDescription>
            </EmptyHeader>
        </Empty>
    )
}

export default Loading