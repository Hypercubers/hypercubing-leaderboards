import { useMemo } from "react"
import { Brush, CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts"
import { ChartContainer, ChartTooltip, type ChartConfig } from "./ui/chart"
import type { FullSolve } from "@/lib/backend"
import { chart_render_time, html_render_time, html_render_date } from "@/lib/utils"
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

    const historyData = useMemo(() => {
        const data = history
            .filter((solve) => (isFMC ? solve.move_count !=0 : solve.speed_cs != null && solve.speed_cs != undefined))
            .map((solve) => ({
                date: new Date(solve.solve_date).getTime(),
                time: solve.speed_cs,
                movecount: solve.move_count ?? 0,
                solver: solve.solver.name,
            }))
            .reverse()
        const colors = new Map(
            [...new Set(data.map((datum) => datum.solver))]
                .map((solver, index) => [
                    solver,
                    `hsl(${(index * 137.508) % 360} 70% 50%)`,
                ]),
        )

        return data.map((datum) => ({
            ...datum,
            solverColor: colors.get(datum.solver),
        }))
    }, [history, isFMC])

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
                                content={({ active, payload }) => {
                                    if (!active || !payload?.length) return null

                                    const row = payload[0].payload

                                    return (
                                        <div className="min-w-40 rounded-lg border bg-background/95 p-3 text-sm shadow-lg backdrop-blur">
                                            <div className="mb-2 flex items-center gap-2">
                                                <span
                                                    className="size-2.5 rounded-full"
                                                    style={{ backgroundColor: row.solverColor }}
                                                />
                                                <span className="font-medium">{row.solver}</span>
                                            </div>

                                            <div className="space-y-1"> 
                                                    <span className="font-semibold tabular-nums">
                                                        {isFMC
                                                            ? row.movecount
                                                            : html_render_time(Number(row.time))}
                                                    </span>
                                                <div className="pt-1 text-xs text-muted-foreground">
                                                    {html_render_date((row.date))}
                                                </div>
                                            </div>
                                        </div>
                                    )
                                }}
                            />
                        <Line
                            dataKey={valueKey}
                            type="stepAfter"
                            stroke="var(--color-time)"
                            strokeWidth={2}
                            dot={({cx, cy, payload}) => {
                                if (cx == null || cy == null) return null
                                return (
                                    <circle
                                        cx={cx}
                                        cy={cy}
                                        r={4}
                                        fill={payload.solverColor}
                                        stroke="none"
                                    />
                                )
                            }}
                            activeDot={({cx, cy, payload}) => {
                                if (cx == null || cy == null) return null
                                return (
                                    <circle
                                        cx={cx}
                                        cy={cy}
                                        r={8}
                                        fill={payload.solverColor}
                                        stroke="none"
                                    />
                                )
                            }}
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
