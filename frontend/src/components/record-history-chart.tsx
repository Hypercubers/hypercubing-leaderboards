import { useEffect, useMemo, useRef, useState } from "react"
import { Brush, CartesianGrid, Line, LineChart, ReferenceArea, XAxis, YAxis } from "recharts"
import { ChartContainer, ChartTooltip, type ChartConfig } from "./ui/chart"
import { Button } from "./ui/button"
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

    const lastIndex = Math.max(0, historyData.length - 1)
    const showDots = historyData.length <= 250
    const [visibleRange, setVisibleRange] = useState({ start: 0, end: lastIndex })
    const [selection, setSelection] = useState<{ start: number; end: number } | null>(null)
    const [selecting, setSelecting] = useState(false)
    const selectionRef = useRef<{ start: number; end: number } | null>(null)
    const selectionFrame = useRef<number | null>(null)
    const chartRef = useRef<HTMLDivElement>(null)

    const dateIndexes = useMemo(() => {
        const indexes = new Map<number, number>()
        historyData.forEach(({ date }, index) => {
            if (!indexes.has(date)) indexes.set(date, index)
        })
        return indexes
    }, [historyData])

    useEffect(() => {
        setVisibleRange({ start: 0, end: Math.max(0, historyData.length - 1) })
        selectionRef.current = null
        setSelection(null)
        setSelecting(false)
        return () => {
            if (selectionFrame.current !== null) {
                cancelAnimationFrame(selectionFrame.current)
                selectionFrame.current = null
            }
        }
    }, [historyData])

    const startIndex = Math.min(visibleRange.start, lastIndex)
    const endIndex = Math.max(startIndex, Math.min(visibleRange.end, lastIndex))
    const dateDomain = useMemo<[number, number]>(() => {
        const [min, max] = historyData
            .slice(startIndex, endIndex + 1)
            .reduce<[number, number]>(
                ([minDate, maxDate], { date }) => [Math.min(minDate, date), Math.max(maxDate, date)],
                [Infinity, -Infinity],
            )
        return [min, max]
    }, [historyData, startIndex, endIndex])

    const getDataIndex = (activeLabel: unknown) => {
        if (activeLabel == null) return -1
        return dateIndexes.get(Number(activeLabel)) ?? -1
    }

    const finishSelection = () => {
        const finalSelection = selectionRef.current
        if (selectionFrame.current !== null) {
            cancelAnimationFrame(selectionFrame.current)
            selectionFrame.current = null
        }
        if (finalSelection && finalSelection.start !== finalSelection.end) {
            setVisibleRange({
                start: Math.min(finalSelection.start, finalSelection.end),
                end: Math.max(finalSelection.start, finalSelection.end),
            })
        }
        selectionRef.current = null
        setSelection(null)
        setSelecting(false)
    }

    useEffect(() => {
        const chartElement = chartRef.current
        if (!chartElement) return

        const handleWheel = (event: globalThis.WheelEvent) => {
            event.preventDefault()
            if (historyData.length < 2) return

            const { start, end } = visibleRange
            const span = end - start
            const maxSpan = historyData.length - 1
            const zoomIn = event.deltaY < 0
            const nextSpan = zoomIn
                ? Math.max(1, Math.floor(span * 0.8))
                : Math.min(maxSpan, Math.ceil(span * 1.25))
            if (nextSpan === span) return

            const bounds = chartElement.getBoundingClientRect()
            if (bounds.width === 0) return
            const pointerPosition = Math.max(0, Math.min(1, (event.clientX - bounds.left) / bounds.width))
            const anchor = start + span * pointerPosition
            let nextStart = Math.round(anchor - nextSpan * pointerPosition)
            let nextEnd = nextStart + nextSpan

            if (nextStart < 0) {
                nextStart = 0
                nextEnd = nextSpan
            } else if (nextEnd > maxSpan) {
                nextEnd = maxSpan
                nextStart = maxSpan - nextSpan
            }

            setVisibleRange({ start: nextStart, end: nextEnd })
        }

        chartElement.addEventListener("wheel", handleWheel, { passive: false })
        return () => chartElement.removeEventListener("wheel", handleWheel)
    }, [historyData.length, visibleRange.start, visibleRange.end])

    const resetRange = () => {
        setVisibleRange({ start: 0, end: lastIndex })
    }

    return (
        <>
            {historyData.length > 0 ?
                <>
                <div ref={chartRef} className="w-full">
                <ChartContainer className="aspect-[32/9]" config={chartConfig}>
                    <LineChart
                        data={historyData}
                        margin={{
                            left: 12,
                            right: 12,
                        }}
                        onMouseDown={(event) => {
                            const index = getDataIndex(event.activeLabel)
                            if (index < 0) return
                            const nextSelection = { start: index, end: index }
                            selectionRef.current = nextSelection
                            setSelection(nextSelection)
                            setSelecting(true)
                        }}
                        onMouseMove={(event) => {
                            if (!selecting) return
                            const index = getDataIndex(event.activeLabel)
                            const currentSelection = selectionRef.current
                            if (index >= 0 && currentSelection && currentSelection.end !== index) {
                                selectionRef.current = { ...currentSelection, end: index }
                                if (selectionFrame.current === null) {
                                    selectionFrame.current = requestAnimationFrame(() => {
                                        selectionFrame.current = null
                                        setSelection(selectionRef.current)
                                    })
                                }
                            }
                        }}
                        onMouseUp={finishSelection}
                        onMouseLeave={finishSelection}
                        >
                        <CartesianGrid
                            strokeDasharray={"3 3"}
                            vertical={false}
                            />
                        <XAxis
                            className="select-none"
                            dataKey={"date"}
                            type="number"
                            domain={dateDomain}
                            allowDataOverflow={true}
                            padding={{ left: 10, right: 10 }}
                            tickFormatter={(value) =>
                                new Date(value).toLocaleDateString(undefined, {
                                    month: "short",
                                    year: "numeric",
                                })
                            }

                            />
                        {selection && historyData[selection.start] && historyData[selection.end] && (
                            <ReferenceArea
                                x1={Math.min(historyData[selection.start].date, historyData[selection.end].date)}
                                x2={Math.max(historyData[selection.start].date, historyData[selection.end].date)}
                                strokeOpacity={0.3}
                                fill="var(--color-chart-4)"
                                fillOpacity={0.1}
                            />
                        )}
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
                            dot={showDots ? ({cx, cy, payload}) => {
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
                            } : false}
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
                            isAnimationActive={false}
                        />
                        {historyData.length > 2 &&
                            <Brush
                                className="select-none"
                                dataKey="date"
                                height={24}
                                padding={{ left: 10, right: 10, top: 0, bottom: 0 }}
                                alwaysShowText
                                startIndex={startIndex}
                                endIndex={endIndex}
                                onChange={({ startIndex: nextStart, endIndex: nextEnd }) => {
                                    setVisibleRange({
                                        start: nextStart ?? 0,
                                        end: nextEnd ?? lastIndex,
                                    })
                                }}
                                stroke="var(--color-chart-4)"
                                fill="var(--color-background)"
                                tickFormatter={(value) => html_render_date(value)}
                            />
                        }
                    </LineChart>
                </ChartContainer>
                </div>
                <div className="mt-2 flex justify-end">
                    <Button type="button" size="sm" variant="outline" onClick={resetRange}>
                        Reset zoom
                    </Button>
                </div>
                </>
            :
            <></>
            }
        </>
    )
}

export default RecordHistoryChart
