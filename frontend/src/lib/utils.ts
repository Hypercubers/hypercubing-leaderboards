import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

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

export function html_render_date(solve_date: string): string {
  let date = new Date(solve_date)
  return date.toISOString().split('T')[0]
}
