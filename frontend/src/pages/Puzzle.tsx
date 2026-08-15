import CategoryQuerySelector from "@/components/category-query-selector";
import FullSolveTable from "@/components/full-solve-table";
import Header from "@/components/header"
import RankedFullSolveTable from "@/components/ranked-full-solve-table";
import RecordHistoryChart from "@/components/record-history-chart";
import { getPuzzleInfo, getPuzzleSolves, type RankedFullSolve, type Puzzle, getRecordHistory, type FullSolve } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { useEffect, useMemo, useState } from "react"
import { useSearchParams } from "react-router-dom"

function Puzzle() {

    const [searchParams] = useSearchParams()
    const id = Number(searchParams.get("id")) || 1
    const [solves, setSolves] = useState<RankedFullSolve[]>([])
    const [history, setHistory] = useState<FullSolve[]>([])
    const [puzzle, setPuzzle] = useState<Puzzle>()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    useEffect(() => {
        getPuzzleInfo(id).then(setPuzzle)
    }, [])

    useEffect(() => {
        getPuzzleSolves(id, categoryQuery).then(setSolves)
        getRecordHistory(id, categoryQuery).then(setHistory)
    }, [id, searchParams.toString()]) // .toString() so the effect keys off actual content

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">{puzzle && `${puzzle.name}`}</h1>

            <CategoryQuerySelector puzzleId={id} query={categoryQuery} />

            {searchParams.get("history") === "true" ? (
                <>
                <RecordHistoryChart history={history}/>
                <FullSolveTable FullSolves={history} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
                </>
            )
            : <RankedFullSolveTable RankedSolves={solves} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
            }


        </>
    )
}

export default Puzzle
