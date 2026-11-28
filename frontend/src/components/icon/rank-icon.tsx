import goldIcon from "@/assets/gold-hypercube.svg"
import silverIcon from "@/assets/silver-hypercube.svg"
import bronzeIcon from "@/assets/bronze-hypercube.svg"

type props = {
    rank: number
}

/**
 *
 * @param rank rank of the solve
 * @returns svg containing the rank icon if the rank is 1, 2, or 3
 */
function RankIcon({ rank }: props) {
    const iconByRank: Record<number, string> = {
        1: goldIcon,
        2: silverIcon,
        3: bronzeIcon,
    }

    const icon = iconByRank[rank]

    return icon ? (
        <img
            src={icon}
            alt={`Rank ${rank}`}
            className="size-5"
        />
    ) : null
}

export default RankIcon
