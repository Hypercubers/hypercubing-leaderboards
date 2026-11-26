import Header from "@/components/header";

interface ErrorProps {
    code?: string;
    msg?: string;
}

function Error({ code, msg }: ErrorProps) {
    return (
        <>
            <Header PageTitle="Error"/>

            <svg
                width="800"
                height="125"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <linearGradient id="rainbow" x1="0" y1="0.5" x2="1" y2="0.7">
                    <stop stopColor="#ff0000" stopOpacity='1' offset='0'/>
                    <stop stopColor='#f7ff00' stopOpacity='1' offset='0.2'/>
                    <stop stopColor='#00ff00' stopOpacity='1' offset='0.5'/>
                    <stop stopColor='#00ffff' stopOpacity='1' offset='0.6'/>
                    <stop stopColor='#0000ff' stopOpacity='1' offset='0.8'/>
                    <stop stopColor='#ff00ff' stopOpacity='1' offset='1'/>
                    </linearGradient>
                </defs>
                <text
                    x="0"
                    y="100"
                    fontWeight="bold"
                    fontStyle="normal"
                    fontVariant="normal"
                    fontStretch="normal"
                    fontSize="125px"
                    fontFamily="'Comic Sans MS'"
                    fill="url(#rainbow)"
                    strokeWidth="5"
                    stroke="black">{code}</text>
                </svg>
                <p>{msg}</p>

        </>
    )
}

export default Error
