import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts"
import { ChartContainer, ChartTooltip, ChartTooltipContent, type ChartConfig } from "./ui/chart"
import type { FullSolve } from "@/lib/backend"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card"
import { html_render_time } from "@/lib/utils"

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

type chartData = {
    date: string,
    time?: string,
    movecount?: string
}

interface props {
    history: FullSolve[]
}

function RecordHistoryChart({history}: props) {

    const historyData = history
    .filter((solve) => solve.speed_cs != null && solve.speed_cs != undefined)
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
                {/* <CardDescription>Shows the record history for this puzzle and event</CardDescription> */}
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    <LineChart data={historyData}>
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
                            dataKey={"time"}
                            domain={["dataMin - 1000", "dataMax + 1000"]}
                            tickFormatter={(value) => html_render_time(Number(value))}
                        />
                        <ChartTooltip
                            cursor={false}
                            content={({active, payload}) => {
                                if (!active || !payload?.length) return null
                                const row = payload[0].payload
                                return (
                                    <div className="rounded-md border bg-background p-2 text-sm shadow-sm">
                                        <div className="font-medium">
                                        {html_render_time(Number(row.time))}
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
                        {/* <ChartTooltip
                            cursor={false}
                            formatter={(value) => html_render_time(Number(value))}
                            labelFormatter={(label) =>
                                new Date(label).toLocaleDateString()
                            }
                        /> */}
                        <Line
                            dataKey="time"
                            type="stepAfter"
                            stroke="var(--color-time)"
                            strokeWidth={4}
                            dot={{fill: "var(--color-time)"}}
                            activeDot={{fill: "var(--color-chart-1)", r:8}}
                        />
                    </LineChart>
                </ChartContainer>
            </CardContent>
        </Card>
    )
}

export default RecordHistoryChart
