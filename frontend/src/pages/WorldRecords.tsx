import CategoryQuerySelector from "@/components/category-query-selector"
import Header from "@/components/header"
import DistinctTable from "@/components/table/distinct-table"
import RecordTable from "@/components/table/record-table"
import { getDistinctRecords, getWorldRecords, type Distinct, type Record } from "@/lib/backend"
import { url_params_to_category_query } from "@/lib/utils"
import { useEffect, useMemo, useState } from "react"
import { useSearchParams } from "react-router-dom"


function WorldRecords() {
    const [searchParams] = useSearchParams();

    const [records, setRecords] = useState<Record[]>([])
    const [distinct, setDistinct] = useState<Distinct[]>([])

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    const isDistinct = categoryQuery.distinct

    useEffect(() => {
        if (isDistinct) {
            getDistinctRecords().then(setDistinct)
        } else {
            getWorldRecords(categoryQuery).then(setRecords)
        }
    }, [categoryQuery, isDistinct])

    return (
        <>
            <Header/>
            <h1 className="text-4xl m-2">World Records</h1>

            <CategoryQuerySelector query={categoryQuery} showAggregates/>

            {categoryQuery.distinct ?
                <DistinctTable Records={distinct}/>
            :
                <RecordTable Records={records} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
            }
        </>
    )

}

export default WorldRecords
