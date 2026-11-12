import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import type { Event, SolveFlags, CategoryQuery, SolveData, FullSolve } from "@/lib/backend";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

/**
 *
 * @param time_cs time of the solve in centiseconds
 * @returns formatted string of the time, truncating to a whole number of seconds
 */
export function chart_render_time(time_cs: number): string {
    const s = Math.trunc((time_cs / 100) % 60);
    const m = Math.trunc((time_cs / (100 * 60)) % 60);
    const h = Math.trunc((time_cs / (100 * 60 * 60)) % 24);
    const d = Math.trunc(time_cs / (100 * 60 * 60 * 24));

    if (d > 0) {
        return (`${d}d ${h.toString().padStart(2, '0')}h ${m.toString().padStart(2, '0')}m ${s.toString().padStart(2, '0')}s`)
    } else if (h > 0) {
        return (`${h}h ${m.toString().padStart(2, '0')}m ${s.toString().padStart(2, '0')}s`)
    } else if (m > 0) {
        return (`${m}m ${s.toString().padStart(2, '0')}s`)
    } else {
        return (`${s}s`)
    }
}

/**
 *
 * @param time_cs time of the solve in centiseconds
 * @returns formatted string of the time
 * @example 123456 -> "12m 34s 56cs"
 */
export function html_render_time(time_cs: number): string {
    const cs = Math.trunc(time_cs % 100);
    const s = Math.trunc((time_cs / 100) % 60);
    const m = Math.trunc((time_cs / (100 * 60)) % 60);
    const h = Math.trunc((time_cs / (100 * 60 * 60)) % 24);
    const d = Math.trunc(time_cs / (100 * 60 * 60 * 24));

    if (d > 0) {
        return (`${d}d ${h.toString().padStart(2, '0')}h ${m.toString().padStart(2, '0')}m ${s.toString().padStart(2, '0')}.${cs.toString().padStart(2, '0')}s`)
    } else if (h > 0) {
        return (`${h}h ${m.toString().padStart(2, '0')}m ${s.toString().padStart(2, '0')}.${cs.toString().padStart(2, '0')}s`)
    } else if (m > 0) {
        return (`${m}m ${s.toString().padStart(2, '0')}.${cs.toString().padStart(2, '0')}s`)
    } else {
        return (`${s}.${cs.toString().padStart(2, '0')}s`)
    }
}

/**
 *
 * @param solve_date string of solve_date from the backend
 * @returns formatted date string in YYYY-MM-DD format
 */
export function html_render_date(solve_date: string): string {
    const date = new Date(solve_date)
    return date.toISOString().split('T')[0]
}

/**
 * 
 * @param solve_date string of date from the backend
 * @returns formatted date and time string in YYYY-MM-DD at HH:MM:SS format
 */
export function html_render_datetime(solve_date: string): string {
    const date = new Date(solve_date)
    return date.toISOString().replace("T", " at ").split(".")[0]
}

/**
 *
 * @param link youtube video link
 * @returns video ID from the link
 * @example "https://youtu.be/j0JWvRVgEek" => "j0JWvRVgEek"
 */
export function get_youtube_id(link: string): string {
    const url = new URL(link)
    const id = url.searchParams.get("v")
    if (id != null) {
        return id
    } else {
        const linkarray = link.split("/")
        return linkarray[linkarray.length-1]
    }
}

export function puz_flags(flags: SolveFlags): string {
    let suffix = ""

    if (flags.average) suffix = "Average"
    if (flags.blind) suffix = "Blindfolded"
    if (flags.one_handed) suffix = "One-handed"
    if (flags.computer_assisted) suffix = "Fewest Moves (computer assisted)"

    return suffix
}


export function puz_name(event: Event): string {
    const name = event.puzzle.name
    let suffix = ""
    // type is Fmc
    if ("computer_assisted" in event.category) {
        if (event.category.Fmc && event.category.Fmc.computer_assisted) {
        suffix = `Fewest Moves (computer assisted)`
        } else {
        suffix = `Fewest Moves`
        }
    }
    // type is Speed
    else {
        if (event.category.Speed && event.category.Speed.average)
        suffix = "Average"
        else if (event.category.Speed && event.category.Speed.blind)
        suffix = "Blindfolded"
        else if (event.category.Speed && event.category.Speed.one_handed)
        suffix = "One-Handed"
    }

    return `${name} ${suffix}`
}

/**
 *
 * @param query the CategoryQuery to decipher
 * @returns a string with all URL paramaters to append to the fetch request
 */
export function category_query_to_url_params(query: CategoryQuery | undefined): string {
    if (query == undefined) return ""

    const params = new URLSearchParams()

    if (query.distinct) {
        params.set("event", "distinct")
        return params.toString()
    }

    if (query.Speed.average) {
        params.set("event", "avg")
    } else if (query.Speed.blind) {
        params.set("event", "bld")
    } else if (query.Speed.one_handed) {
        params.set("event", "oh")
    } else if (query.Fmc.enabled) {
        params.set("event", query.Fmc.computer_assisted ? "fmcca" : "fmc")
    } else {
        params.delete("event")
    }

    if (query.Speed.filters !== undefined) {
        params.set("filters", String(query.Speed.filters))
    }

    if (query.Speed.macros !== undefined) {
        params.set("macros", String(query.Speed.macros))
    }

    const variant = query.Speed.variant ?? "Default"
    const program = query.Speed.program ?? "Default"

    // hardcoded variant/program combinations
    if (variant === "phys" && program === "virtual") {
        params.set("variant", "phys")
        params.set("program", "virtual")
    } else if (variant === "phys") {
        params.set("variant", "phys")
    } else if (variant === "1d") {
        params.set("variant", "1d")
    } else if (program === "material") {
        params.set("program", "material")
    } else if (variant !== "Default" && variant !== "") {
        params.set("variant", variant)
    } else if (program !== "Default" && program !== "") {
        params.set("program", program)
    }

    return params.toString()
}

export function url_params_to_category_query(params: URLSearchParams): CategoryQuery {
    const event = params.get("event")
    const variant = params.get("variant") ?? "Default"
    const program = params.get("program") ?? "Default"

    const normalizedVariant = variant === "" ? "Default" : variant
    const normalizedProgram = program === "" ? "Default" : program

    return {
        distinct: event === "distinct",
        Speed: {
            average: event === "avg",
            blind: event === "bld",
            one_handed: event === "oh",
            filters: params.has("filters") ? params.get("filters") === "true" : undefined,
            macros: params.has("macros") ? params.get("macros") === "true" : undefined,
            variant: normalizedVariant,
            program: normalizedProgram,
        },
        Fmc: {
            enabled: event === "fmc" || event === "fmcca",
            computer_assisted: event === "fmcca",
        },
    }
}


export function createEmptySolveData(solverId?: number): SolveData {
    return {
        solve_id: undefined, puzzle_id: 0, variant_id: undefined, program_id: 0,
        solver_id: solverId, solve_date: '', solver_notes: '', moderator_notes: '',
        solve_h: undefined, solve_m: undefined, solve_s: undefined, solve_cs: undefined,
        uses_filters: true, uses_macros: false, average: false, one_handed: false, blind: false,
        memo_h: undefined, memo_m: undefined, memo_s: undefined, memo_cs: undefined,
        video_url: undefined, move_count: undefined, computer_assisted: false,
        replace_log_file: false, log_file: undefined,
    }
}

export function splitCentiseconds(value: number | undefined) {
    if (value === undefined) {
        return {
            hours: undefined,
            minutes: undefined,
            seconds: undefined,
            centiseconds: undefined,
        }
    }

    const centiseconds = value % 100
    const totalSeconds = Math.floor(value / 100)
    const seconds = totalSeconds % 60
    const totalMinutes = Math.floor(totalSeconds / 60)

    return {
        hours: Math.floor(totalMinutes / 60) || undefined,
        minutes: totalMinutes % 60 || undefined,
        seconds: seconds || undefined,
        centiseconds: centiseconds || undefined,
    }
}


export function solveToFormValues(solve: FullSolve): SolveData {
    const speed = splitCentiseconds(solve.speed_cs)
    const memo = splitCentiseconds(solve.memo_cs)

    return {
        solve_id: solve.id,
        puzzle_id: solve.puzzle.id,
        variant_id: solve.variant?.id,
        program_id: solve.program.id,
        solver_id: solve.solver.id,
        solve_date: solve.solve_date.slice(0, 10),
        solver_notes: solve.solver_notes ?? "",
        moderator_notes: solve.moderator_notes ?? "",
        solve_h: speed.hours,
        solve_m: speed.minutes,
        solve_s: speed.seconds,
        solve_cs: speed.centiseconds,
        uses_filters: solve.flags.filters,
        uses_macros: solve.flags.macros,
        average: solve.flags.average,
        one_handed: solve.flags.one_handed,
        blind: solve.flags.blind,
        memo_h: memo.hours,
        memo_m: memo.minutes,
        memo_s: memo.seconds,
        memo_cs: memo.centiseconds,
        video_url: solve.video_url ?? undefined,
        move_count: solve.move_count ?? undefined,
        computer_assisted: solve.flags.computer_assisted,
        replace_log_file: false,
        log_file: undefined,
    }
}