import CategoryQuerySelector from "@/components/category-query-selector";
import Header from "@/components/header";
import PbTable from "@/components/table/pb-table";
import { Button } from "@/components/ui/button";
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu";
import { useAuth } from "@/lib/auth-context";
import { getUser, getUserPbs, type PublicUser, type PB } from "@/lib/backend";
import { url_params_to_category_query } from "@/lib/utils";
import { Gavel, ListTodo } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";


interface params {
    id: number;
}

function Solver() {
    const [searchParams] = useSearchParams()
    const [solverName, setSolverName] = useState<PublicUser|null>(null)
    const query: params = {
        id: Number(searchParams.get('id')) || 1
    }

    const { user } = useAuth()
    const navigate = useNavigate()

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
            <Header PageTitle={solverName?.name ?? `Solver ${query.id}`}/>

            {user?.moderator && 
                <NavigationMenu className="m-2">
                    <NavigationMenuList className="flex-wrap">
                        <NavigationMenuItem>
                        <a href="https://hypercubing.xyz/"><Button><ListTodo/> Pending submissions</Button></a>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                        <Button onClick={() => navigate(`/audit-log/user?id=${query.id}`)}><Gavel/> User audit log</Button>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            }

            <CategoryQuerySelector query={categoryQuery}/>

            <PbTable PBs={solves} isFmc={searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"}/>
        </>
    )
}

export default Solver;
