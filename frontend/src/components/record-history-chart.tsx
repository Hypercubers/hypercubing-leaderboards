import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts"
import { ChartContainer, ChartTooltip, type ChartConfig } from "./ui/chart"
import type { FullSolve } from "@/lib/backend"
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card"
import { html_render_time } from "@/lib/utils"
import { useSearchParams } from "react-router-dom"

const chartConfig = {
    time: {
        label: "Time",
        color: "var(--chart-4)",
    },
    movecount: {
        label: "Moves",
        color: "var(--chart-2)",
    },
} satisfies ChartConfig

interface props {
    history: FullSolve[]
}

function RecordHistoryChart({history}: props) {
    const [searchParams] = useSearchParams()

    const isFMC = searchParams.get("event") === "fmc" || searchParams.get("event") === "fmcca"

    const historyData = history
    .filter((solve) => (isFMC ? solve.move_count !=0 : solve.speed_cs != null && solve.speed_cs != undefined))
    .map((solve) => (
        {
            date: new Date(solve.solve_date).getTime(),
            time: solve.speed_cs,
            movecount: solve.move_count ?? 0,
            solver: solve.solver.name
        }
    )).reverse()

    return (
        <Card>
            <CardHeader>
                <CardTitle>Record History</CardTitle>
            </CardHeader>
            <CardContent>
                {historyData.length > 0 ?
                    <ChartContainer config={chartConfig}>
                        <LineChart
                            data={historyData}
                            margin={{
                                left: 12,
                                right: 12,
                            }}
                            >
                            <CartesianGrid
                                strokeDasharray={"3 3"}
                                vertical={false}
                            />
                            <XAxis
                                dataKey={"date"}
                                type="number"
                                domain={["dataMin", "dataMax"]}
                                tickFormatter={(value) =>
                                new Date(value).toLocaleDateString(undefined, {
                                    month: "short",
                                    year: "numeric",
                                    })
                                }

                            />
                            <YAxis
                                dataKey={isFMC ? "movecount" : "time"}
                                domain={[`dataMin - ${isFMC ? "5" : "5000"}`, `dataMax + ${isFMC ? "5" : "5000"}`]}
                                // tickCount={6}
                                tickFormatter={(value: number) => {
                                    if (!isFMC) {
                                        const roundedCs = Math.round(Number(value) / 500) * 500
                                        return html_render_time(roundedCs)
                                    } else {
                                        return value.toString()
                                    }
                                }}
                            />
                            <ChartTooltip
                                cursor={false}
                                content={({active, payload}) => {
                                    if (!active || !payload?.length) return null
                                    const row = payload[0].payload
                                    return (
                                        <div className="rounded-md border bg-background p-2 text-sm shadow-sm">
                                            <div className="font-medium">
                                            {isFMC ? `${row.movecount} moves` : html_render_time(Number(row.time))}
                                            </div>
                                            <div className="text-muted-foreground">
                                            {new Date(row.date).toLocaleDateString(undefined, {
                                                month: "numeric",
                                                day: "numeric",
                                                year: "numeric",
                                            })}
                                            </div>
                                            <div className="text-muted-foreground">
                                                {row.solver}
                                            </div>

                                        </div>
                                    )
                                    }
                                }
                            />
                            <Line
                                dataKey={isFMC ? "movecount" : "time"}
                                type="stepAfter"
                                stroke="var(--color-time)"
                                strokeWidth={4}
                                dot={{fill: "var(--color-time)"}}
                                activeDot={{fill: "var(--color-chart-1)", r:8}}
                            />
                        </LineChart>
                    </ChartContainer>
                :
                    <p>no data to display</p>
                }
            </CardContent>
        </Card>
    )
}

export default RecordHistoryChart
