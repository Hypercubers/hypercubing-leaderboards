import { useMemo } from "react"
import { Brush, CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts"
import { ChartContainer, ChartTooltip, type ChartConfig } from "./ui/chart"
import type { FullSolve } from "@/lib/backend"
import { chart_render_time, html_render_time } from "@/lib/utils"
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

    const historyData = useMemo(() => history
    .filter((solve) => (isFMC ? solve.move_count !=0 : solve.speed_cs != null && solve.speed_cs != undefined))
    .map((solve) => (
        {
            date: new Date(solve.solve_date).getTime(),
            time: solve.speed_cs,
            movecount: solve.move_count ?? 0,
            solver: solve.solver.name
        })).reverse(),
    [history, isFMC])

    const valueKey: "movecount" | "time" = isFMC ? "movecount" : "time"

    return (
        <>
            {historyData.length > 0 ?
                <ChartContainer className="aspect-[32/9]" config={chartConfig}>
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
                            className="select-none"
                            dataKey={"date"}
                            type="number"
                            domain={["dataMin", "dataMax"]}
                            padding={{ left: 10, right: 10 }}
                            tickFormatter={(value) =>
                                new Date(value).toLocaleDateString(undefined, {
                                    month: "short",
                                    year: "numeric",
                                })
                            }

                            />
                        <YAxis
                            className="select-none"
                            dataKey={valueKey}
                            domain={isFMC ? [0, "dataMax + 5"] : ["dataMin - 500", "dataMax + 500"]}
                            padding={{ top: 10, bottom: 10 }}
                            width={isFMC ? 30 : 90}
                            tickFormatter={(value: number) => {
                                if (!isFMC) {
                                    const roundedCs = Math.round(Number(value) / 500) * 500
                                    return chart_render_time(roundedCs)
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
                                    <div className="rounded-md border bg-background p-2 text-sm shadow-sm select-none">
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
                            dataKey={valueKey}
                            type="stepAfter"
                            stroke="var(--color-time)"
                            strokeWidth={4}
                            dot={{fill: "var(--color-time)"}}
                            activeDot={{fill: "var(--color-chart-1)", r:8}}
                            animationDuration={300}
                            isAnimationActive
                            />
                        <Brush
                            className="select-none"
                            dataKey="date"
                            height={24}
                            stroke="var(--color-chart-4)"
                            fill="var(--color-background)"
                            tickFormatter={(value) =>
                                new Date(value).toLocaleDateString(undefined, {
                                    month: "short",
                                    year: "numeric",
                                })
                            }
                        />
                    </LineChart>
                </ChartContainer>
            :
            <></>
            }
        </>
    )
}

export default RecordHistoryChart
