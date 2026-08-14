import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import type { Event, Category, Speed, Fmc, SolveFlags, CategoryQuery } from "@/lib/backend";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

// returns a string of formatted time from a number of centiseconds
export function html_render_time(time_cs: number): string {
    let cs = Math.trunc(time_cs % 100);
    let s = Math.trunc((time_cs / 100) % 60);
    let m = Math.trunc((time_cs / (100 * 60)) % 60);
    let h = Math.trunc((time_cs / (100 * 60 * 60)) % 24);
    let d = Math.trunc(time_cs / (100 * 60 * 60 * 24));

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
    let date = new Date(solve_date)
    return date.toISOString().split('T')[0]
}

// returns the video ID string from a YouTube link string
// i.e. https://youtu.be/j0JWvRVgEek => j0JWvRVgEek
export function get_youtube_id(link: string): string {
    let url = new URL(link)
    let id = url.searchParams.get("v")
    if (id != null) {
        return id
    } else {
        let linkarray = link.split("/")
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
    let name = event.puzzle.name
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
    if (query == undefined) {
        return (
            ""
        )
    }


    let params: URLSearchParams = new URLSearchParams

    if (query.Speed.average) params.set("event", "avg")
    if (query.Speed.blind) params.set("event", "bld")
    if (query.Speed.one_handed) params.set("event", "oh")

    if (query.Fmc.computer_assisted) {
        params.set("event", "fmcca")
    } else {
        params.set("event", "fmc")
    }

    if (query.Speed.filters !== undefined) {
        params.set("filters", query.Speed.filters?"true":"false")
    }

    if (query.Speed.macros !== undefined) {
        params.set("macros", query.Speed.macros?"true":"false")
    }

    return params.toString()
}
