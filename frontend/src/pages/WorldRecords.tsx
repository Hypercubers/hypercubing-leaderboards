import CategoryQuerySelector from "@/components/category-query-selector"
import Header from "@/components/header"
import DistinctTable from "@/components/table/distinct-table"
import RecordTable from "@/components/table/record-table"
import { Button } from "@/components/ui/button"
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu"
import { useAuth } from "@/lib/auth-context"
import { getDistinctRecords, getWorldRecords, type Distinct, type Record } from "@/lib/backend"
import { url_params_to_category_query } from "@/lib/utils"
import { Gavel, ListTodo, Shapes, UsersRound } from "lucide-react"
import { useEffect, useMemo, useState } from "react"
import { useNavigate, useSearchParams } from "react-router-dom"


function WorldRecords() {
    const [searchParams] = useSearchParams();

    const [records, setRecords] = useState<Record[]>([])
    const [distinct, setDistinct] = useState<Distinct[]>([])

    const { user } = useAuth()

    const navigate = useNavigate()

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
            <Header PageTitle="World Records" excludeBreadcrumbs/>

            {user?.moderator && 
                    <NavigationMenu className="m-2">
                    <NavigationMenuList className="flex-wrap">
                        <NavigationMenuItem>
                        <a href="https://hypercubing.xyz/"><Button><ListTodo/> Pending submissions</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <a href="https://hypercubing.xyz/faq/"><Button><Gavel/> General audit log</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Button onClick={() => navigate("/")}><Shapes/> Categories</Button>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Button onClick={() => navigate("/")}><UsersRound/> Users</Button>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            }

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
