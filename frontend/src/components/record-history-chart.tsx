import { Bar, BarChart } from "recharts"
import { ChartContainer, type ChartConfig } from "./ui/chart"

const chartConfig = {
    desktop: {
        label: "Desktop",
        color: "var(--chart-4)",
    },
    mobile: {
        label: "Mobile",
        color: "#60a5fa",
    },
} satisfies ChartConfig

const chartData = [
        { month: "January", desktop: 186, mobile: 80 },
        { month: "February", desktop: 305, mobile: 200 },
        { month: "March", desktop: 237, mobile: 120 },
        { month: "April", desktop: 73, mobile: 190 },
        { month: "May", desktop: 209, mobile: 130 },
        { month: "June", desktop: 214, mobile: 140 },
    ]

function RecordHistoryChart() {
    return (
        <ChartContainer config={chartConfig}>
            <BarChart data={chartData}>
                <Bar dataKey="desktop" fill="var(--color-desktop)" radius={4} />
            </BarChart>

        </ChartContainer>
    )
}

export default RecordHistoryChart
