import Header from "@/components/header";


function FourZeroFour() {
    return (
        <>
            <Header/>

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
                    font-weight="bold"
                    font-style="normal"
                    font-variant="normal"
                    font-stretch="normal"
                    font-size="125px"
                    font-family="'Comic Sans MS'"
                    fill="url(#rainbow)"
                    stroke-width="5"
                    stroke="black">404</text>
                </svg>

        </>
    )
}

export default FourZeroFour
