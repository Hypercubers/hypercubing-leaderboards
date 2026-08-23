import { Skeleton } from "./ui/skeleton"
import { TableCell, TableRow } from "./ui/table"


interface props  {
    rows: number,
    cols: number
}

// returns several TableRows filled with Skeleton (visual indicator of loading data)
function SkeletonTableRows({rows, cols}: props) {
    return (
        <>
        {
            Array.from({length: rows}).map(() => (
                <TableRow>
                    {
                        Array.from({length: cols}).map(() => (
                            <TableCell><Skeleton className="w-full h-4"></Skeleton></TableCell>
                        ))
                    }
                </TableRow>
            )
        )
        }

        </>
    )
}

export default SkeletonTableRows
