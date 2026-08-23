import { Card, CardHeader, CardTitle, CardContent } from "./ui/card"
import SkeletonTableRows from "@/components/skeleton-table-rows"
import { Table, TableBody } from "./ui/table"
import { Skeleton } from "./ui/skeleton"

function SkeletonSolveCard() {
    return (
         <Card>
            <CardHeader>
                <CardTitle>
                    <Skeleton className="h-4 w-1/3" />
                </CardTitle>
            </CardHeader>
            <CardContent>
                <Skeleton className="aspect-video w-[560px]" />
                <Table className="w-1/2">
                    <TableBody>
                        <SkeletonTableRows rows={7} cols={2}></SkeletonTableRows>
                    </TableBody>
                </Table>

            </CardContent>
        </Card>
    )
}

export default SkeletonSolveCard
