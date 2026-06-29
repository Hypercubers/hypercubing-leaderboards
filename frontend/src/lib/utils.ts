import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

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
