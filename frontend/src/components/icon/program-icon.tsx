
type props = {
    abbr: string
}

function ProgramIcon({abbr}: props) {
    return (
        <svg width="1.6rem" height="1.6rem">
            <image href={`https://assets.hypercubing.xyz/img/icons/programs/${abbr}.svg`} width="1.6rem" height="1.6rem"/>
        </svg>
    )
}

export default ProgramIcon
