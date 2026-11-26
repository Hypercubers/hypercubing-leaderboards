import CategoryQuerySelector from "@/components/category-query-selector";
import ErrorData from "@/components/error-data";
import Header from "@/components/header";
import Loading from "@/components/loading";
import PbTable from "@/components/table/pb-table";
import { Button } from "@/components/ui/button";
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu";
import { useAuth } from "@/lib/auth-context";
import { getUser, getUserPbs, type PublicUser } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { Gavel, ListTodo } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { Link, useSearchParams } from "react-router-dom";
import Error from "./Error";
function Solver() {
    
    const [solver, setsolver] = useState<PublicUser|null>(null)

    const [searchParams] = useSearchParams()
    const id = Number(searchParams.get("id"))

    if (!id) {
        return <Error code="404" msg="Not Found. No solver ID provided."/>
    }

    const { user } = useAuth()

    const categoryQuery = useMemo(
        () => url_params_to_category_query(searchParams),
        [searchParams]
    )

    useEffect(() => {
        getUser(id).then(setsolver)
    }, [])

    const solverPbsQuery = useQuery({
        queryKey: ["solver-pbs", id, categoryQuery],
        queryFn: () => getUserPbs(id, categoryQuery),
        enabled: !!id,
    })

    if (!solver) {
        return <Error code="404" msg="Not Found. No solver found for the ID provided."/>
    }

    return (
        <>
            <Header PageTitle={solver?.name ?? `Solver ${id}`}/>

            {user?.moderator && 
                <NavigationMenu className="m-2">
                    <NavigationMenuList className="flex-wrap">
                        <NavigationMenuItem>
                            <Link to={`/solver-submissions?id=${id}`}><Button><ListTodo/> User submissions</Button></Link>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <Link to={`/audit-log/user?id=${id}`}><Button><Gavel/> User audit log</Button></Link>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            }

            <CategoryQuerySelector query={categoryQuery}/>

            {solverPbsQuery.isPending? (
                <Loading/>
            ) : solverPbsQuery.isError ? (
                <ErrorData word="solves"/>
            ) : (
                <PbTable PBs={solverPbsQuery.data ?? []} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
            )}

        </>
    )
}

export default Solver;
