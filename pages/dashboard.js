import Card from "../components/Card/Card"
import { useState, useEffect} from "react"
import Form from "../components/Form/Form"
import Navbar from "../components/Navbar/Navbar"
import Viewport from "../components/Viewport/Viewport"

export default function Dashboard() {
    const [plants, setPlants] = useState([])

    let Plants = plants.map((plant)=>{
        return <Card img={plant.img} active_growth_period={plant["Active Growth Period"]} lifespan={plant["Lifespan"]} precipitation={plant["Precipitation, Minimum"]} bloom_period={plant["Bloom Period"]} scientific_name={plant["Scientific Name with Author"]} generic_name={plant["Common Name"]}></Card>})

    return <>
    <main style={{height:"100vh"}}>
        <Navbar/>
        <Viewport>
            <section style={{height: "100%", borderRight:"1px solid black"}}>
                <Form setPlants={setPlants} plants={plants}/>
            </section>
            <section style={{display:"flex", alignItems:"flex-start", justifyContent:"flex-start", height:"100%", width:"100%"}}>
                {Plants}
            </section>
        </Viewport>
    </main>
    </>
}