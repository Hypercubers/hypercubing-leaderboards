import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import type { Event, SolveFlags, CategoryQuery } from "@/lib/backend";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

// returns a string of formatted time from a number of centiseconds
export function html_render_time(time_cs: number): string {
    const cs = Math.trunc(time_cs % 100);
    const s = Math.trunc((time_cs / 100) % 60);
    const m = Math.trunc((time_cs / (100 * 60)) % 60);
    const h = Math.trunc((time_cs / (100 * 60 * 60)) % 24);
    const d = Math.trunc(time_cs / (100 * 60 * 60 * 24));

    if (d > 0) {
        return (`${d}d ${h}h ${m}m ${s}.${cs}s`)
    } else if (h > 0) {
        return (`${h}h ${m}m ${s}.${cs}s`)
    } else if (m > 0) {
        return (`${m}m ${s}.${cs}s`)
    } else {
        return (`${s}.${cs}s`)
    }
}

// renders the Date object in YYYY-MM-DD
export function html_render_date(solve_date: string): string {
    const date = new Date(solve_date)
    return date.toISOString().split('T')[0]
}

// returns the video ID string from a YouTube link string
// i.e. https://youtu.be/j0JWvRVgEek => j0JWvRVgEek
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
    if (flags.computer_assissted) suffix = "Fewest Moves (computer assisted)"

    return suffix
}


export function puz_name(event: Event): string {
    const name = event.puzzle.name
    let suffix = ""
    // type is Fmc
    if ("computer_assissted" in event.category) {
        if (event.category.Fmc && event.category.Fmc.computer_assissted) {
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
