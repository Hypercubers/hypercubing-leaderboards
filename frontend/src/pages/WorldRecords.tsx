import Header from "@/components/header"
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { getPuzzles, type Puzzle } from "@/lib/backend"
import { useEffect, useState } from "react"


function WorldRecords() {

    const [puzzles, setPuzzles] = useState<Puzzle[]>([])

      useEffect(() => {
        getPuzzles().then(setPuzzles)
      }, [])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">World Records</h1>
            
            <Table>
                <TableCaption>A list of world records</TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead>ID</TableHead>
                        <TableHead>Name</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {puzzles.length > 0 ? puzzles.map((puzzle) => (
                        <TableRow>
                            <TableCell>{puzzle.id}</TableCell>
                            <TableCell>{puzzle.name}</TableCell>
                        </TableRow>
                    ))
                    :
                    <p>no puzzles loaded</p>
                    }
                </TableBody>
            </Table>
        </>
    )

}

export default WorldRecords
