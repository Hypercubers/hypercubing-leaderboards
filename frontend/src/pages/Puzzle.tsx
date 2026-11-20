import CategoryQuerySelector from "@/components/category-query-selector";
import FullSolveTable from "@/components/table/full-solve-table";
import Header from "@/components/header"
import RankedFullSolveTable from "@/components/table/ranked-full-solve-table";
import RecordHistoryChart from "@/components/record-history-chart";
import { getPuzzleInfo, getPuzzleSolves, type Puzzle, getRecordHistory } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { useMemo } from "react"
import { useSearchParams } from "react-router-dom"
import Error from "./Error";
import { useQuery } from "@tanstack/react-query";
import Loading from "@/components/loading";
import ErrorData from "@/components/error-data";

function Puzzle() {

    const [searchParams] = useSearchParams()
    const id = Number(searchParams.get("id")) || 1

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    const puzzleInfoQuery = useQuery({
        queryKey: ["puzzle-info", id],
        queryFn: () => getPuzzleInfo(id),
    })

    const puzzle = puzzleInfoQuery.data

    const isFmc = searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"

    const historyEnabled = searchParams.get("history") === "true"

    const solvesQuery = useQuery({
        queryKey: ["puzzle-solves", id, searchParams.toString()],
        queryFn: () => getPuzzleSolves(id, categoryQuery),
    })

    const solves = solvesQuery.data ?? []

    const historyQuery = useQuery({
        queryKey: ["puzzle-history", id, searchParams.toString()],
        queryFn: () => getRecordHistory(id, categoryQuery),
        enabled: historyEnabled,
    })

    const history = historyQuery.data ?? []


    return (
        puzzleInfoQuery.isPending ? (
            <>
                <Header PageTitle="Loading..." />
                <Loading/>
            </>
        ) : puzzleInfoQuery.isError ? (
            <Error code={"500"} msg="Error loading puzzle info"/>
        ) : (
            <>
                <Header PageTitle={puzzle?.name ?? `Puzzle ${id}`} />
            
                <CategoryQuerySelector puzzleId={id} query={categoryQuery} />

                {historyEnabled ? (
                    historyQuery.isPending ? (
                        <Loading/>
                    ) : historyQuery.isError ? (
                        <ErrorData word="history" />
                    ) : (
                        <>
                            <RecordHistoryChart history={history}/>
                            <FullSolveTable FullSolves={history} isFmc={isFmc}/>
                        </>
                    )
                ) : solvesQuery.isPending ? (
                    <Loading/>
                ) : solvesQuery.isError ? (
                    <ErrorData word="solves" />
                ) : (
                    <RankedFullSolveTable RankedSolves={solves} isFmc={isFmc}/>
                )}
        </>
        ) 
    )
}

export default Puzzle
