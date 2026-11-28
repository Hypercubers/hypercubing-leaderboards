import CategoryQuerySelector from "@/components/category-query-selector"
import ErrorData from "@/components/error-data"
import Header from "@/components/header"
import Loading from "@/components/loading"
import DistinctTable from "@/components/table/distinct-table"
import RecordTable from "@/components/table/record-table"
import { Button } from "@/components/ui/button"
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu"
import { useAuth } from "@/lib/auth-context"
import { getDistinctRecords, getWorldRecords } from "@/lib/api/solves"
import { url_params_to_category_query } from "@/lib/utils"
import { useQuery } from "@tanstack/react-query"
import { Gavel, ListTodo, Shapes, UsersRound } from "lucide-react"
import { useMemo } from "react"
import { Link, useSearchParams } from "react-router-dom"


function WorldRecords() {
    const [searchParams] = useSearchParams();

    const { user } = useAuth()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    const isDistinct = categoryQuery.distinct

    const worldRecordsQuery = useQuery({
        queryKey: ["world-records", searchParams.toString()],
        queryFn: () => getWorldRecords(categoryQuery),
        enabled: !isDistinct,
    })
    
    const distinctRecordsQuery = useQuery({
        queryKey: ["distinct-records"],
        queryFn: () => getDistinctRecords(),
        enabled: isDistinct,
    })

    return (
        <>
            <Header PageTitle="World Records" excludeBreadcrumbs/>

            {user?.moderator && 
                <NavigationMenu className="m-2">
                    <NavigationMenuList className="flex-wrap">
                        <NavigationMenuItem>
                        <Link to="/pending-submissions"><Button><ListTodo/> Pending submissions</Button></Link>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Link to="/audit-log/general"><Button><Gavel/> General audit log</Button></Link>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Link to="/categories"><Button><Shapes/> Categories</Button></Link>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Link to="/users"><Button><UsersRound/> Users</Button></Link>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            }

            <CategoryQuerySelector query={categoryQuery} showAggregates/>

            {isDistinct ?
                <>
                    {distinctRecordsQuery.isPending? (
                        <Loading/>
                    ) : distinctRecordsQuery.isError ? (
                        <ErrorData word="distinct records" />
                    ) : (
                        <DistinctTable Records={distinctRecordsQuery.data ?? []}/>
                    )}
                   
                </>
            :
                <>
                    {worldRecordsQuery.isPending? (
                        <Loading/>
                    ) : worldRecordsQuery.isError ? (
                        <ErrorData word="world records" />
                    ) : (
                        <RecordTable Records={worldRecordsQuery.data ?? []} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
                    )}
                </>
            }
        </>
    )

}

export default WorldRecords
