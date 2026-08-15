import { useCallback, useMemo, useState } from "react"
import { CartesianGrid, Line, LineChart, ReferenceArea, XAxis, YAxis, type MouseHandlerDataParam } from "recharts"
import { ChartContainer, ChartTooltip, type ChartConfig } from "./ui/chart"
import type { FullSolve } from "@/lib/backend"
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card"
import { Button } from "./ui/button"
import { chart_render_time, html_render_time } from "@/lib/utils"
import { useSearchParams } from "react-router-dom"
import { ZoomOut } from "lucide-react"

type ZoomAndHighlightState = {
    left: string | number;
    right: string | number;
    refAreaLeft: string | number | undefined;
    refAreaRight: string | number | undefined;
    top: string | number;
    bottom: string | number;
    animation: boolean;
};

type AxisDomain = [string | number, string | number];

const getInitialState = (offset: number): ZoomAndHighlightState => ({
    left: "dataMin",
    right: "dataMax",
    refAreaLeft: undefined,
    refAreaRight: undefined,
    top: `dataMax + ${offset}`,
    bottom: `dataMin - ${offset}`,
    animation: true,
})



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
    const yOffset = isFMC ? 5 : 500

    const initialState = useMemo(() => getInitialState(yOffset), [yOffset])
    const [zoomGraph, setZoomGraph] = useState<ZoomAndHighlightState>(initialState)

    const getAxisYDomain = useCallback((
        from: string | number | undefined,
        to: string | number | undefined,
        ref: "time" | "movecount",
        offset: number,
    ): AxisDomain => {
        if (from != null && to != null) {
            const [fromDate, toDate] = Number(from) <= Number(to)
                ? [Number(from), Number(to)]
                : [Number(to), Number(from)]
            const refData = historyData.filter((datum) => datum.date >= fromDate && datum.date <= toDate)
            const firstDatum = refData[0]
            if (firstDatum == null) {
                return [initialState.bottom, initialState.top]
            }
            let bottom = Number(firstDatum[ref] ?? 0)
            let top = Number(firstDatum[ref] ?? 0)
            refData.forEach((d) => {
                const pointValue = Number(d[ref] ?? 0)
                if (pointValue > top) top = pointValue
                if (pointValue < bottom) bottom = pointValue
            })

            return [Math.floor(bottom) - offset, Math.ceil(top) + offset]
        }
        return [initialState.bottom, initialState.top]
    }, [historyData, initialState.bottom, initialState.top])

    const zoom = useCallback(() => {
        setZoomGraph((prev) => {
            let { refAreaLeft, refAreaRight } = prev

            if (refAreaLeft == null || refAreaRight == null || refAreaLeft === refAreaRight) {
                return {
                    ...prev,
                    refAreaLeft: undefined,
                    refAreaRight: undefined,
                }
            }

            if (refAreaLeft > refAreaRight) {
                ;[refAreaLeft, refAreaRight] = [refAreaRight, refAreaLeft]
            }

            const [bottom, top] = getAxisYDomain(refAreaLeft, refAreaRight, valueKey, yOffset)

            return {
                ...prev,
                refAreaLeft: undefined,
                refAreaRight: undefined,
                left: refAreaLeft,
                right: refAreaRight,
                bottom,
                top,
                animation: true,
            }
        })
    }, [getAxisYDomain, valueKey, yOffset])

    const zoomOut = useCallback(() => {
        setZoomGraph(getInitialState(yOffset))
    }, [yOffset])

    const onMouseDown = useCallback((e: MouseHandlerDataParam) => {
        if (typeof e.activeLabel !== "number") return
        setZoomGraph((prev) => ({ ...prev, refAreaLeft: e.activeLabel, refAreaRight: undefined }))
    }, [])

    const onMouseMove = useCallback((e: MouseHandlerDataParam) => {
        if (typeof e.activeLabel !== "number") return
        setZoomGraph((prev) => {
            if (prev.refAreaLeft != null) {
                return { ...prev, refAreaRight: e.activeLabel }
            }
            return prev
        })
    }, [])

    const { refAreaLeft, refAreaRight, left, right, top, bottom, animation } = zoomGraph

    const isDefaultZoom =
        zoomGraph.left === "dataMin" &&
        zoomGraph.right === "dataMax" &&
        zoomGraph.refAreaLeft == null &&
        zoomGraph.refAreaRight == null;

    return (
        <Card>
            <CardHeader>
                    <CardTitle>Record History</CardTitle>

                    {historyData.length > 0 ?
                    <>
                        <CardDescription>Click and drag your mouse horizontally to make a selection to zoom in.</CardDescription>
                        <CardAction>
                            <Button disabled={isDefaultZoom} variant="outline" size="sm" onClick={zoomOut}>
                                <ZoomOut/>
                                Reset zoom
                            </Button>
                        </CardAction>
                    </>
                    :
                    <></>
                    }


            </CardHeader>
            <CardContent>
                {historyData.length > 0 ?
                    <ChartContainer config={chartConfig}>
                        <LineChart
                            data={historyData}
                            onMouseDown={onMouseDown}
                            onMouseMove={onMouseMove}
                            onMouseUp={zoom}
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
                                allowDataOverflow
                                domain={[left, right]}
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
                                allowDataOverflow
                                domain={isFMC ? [0, top] : [bottom, top]}
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
                                isAnimationActive={animation}
                            />
                            {refAreaLeft != null && refAreaRight != null ? (
                                <ReferenceArea
                                    x1={refAreaLeft}
                                    x2={refAreaRight}
                                    strokeOpacity={0.3}
                                    stroke="var(--border)"
                                    fill="var(--color-time)"
                                    fillOpacity={0.15}
                                />
                            ) : null}
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
