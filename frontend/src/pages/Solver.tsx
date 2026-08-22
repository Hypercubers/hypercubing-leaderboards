import CategoryQuerySelector from "@/components/category-query-selector";
import Header from "@/components/header";
import PbTable from "@/components/table/pb-table";
import { getUser, getUserPbs, type PublicUser, type PB } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";


interface params {
    id: number;
}

function Solver() {
    const [searchParams] = useSearchParams()
    const [solverName, setSolverName] = useState<PublicUser|null>(null)
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    const [solves, setSolves] = useState<[PB]>()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    useEffect(() => {
        getUser(query.id).then(setSolverName)
    }, [])

    useEffect(() => {
        getUserPbs(query.id, categoryQuery).then(setSolves)
    }, [categoryQuery])

    return (
        <>
            <Header/>

            <h1 className="text-4xl m-2">{solverName?.name ?? `Solver ${query.id}`}</h1>

            <CategoryQuerySelector query={categoryQuery}/>

            <PbTable PBs={solves} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
        </>
    )
}

export default Solver;
