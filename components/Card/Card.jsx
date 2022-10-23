
export default function Card ({img, active_growth_period, lifespan, precipitation, bloom_period, scientific_name, generic_name}) {
    console.log(img, active_growth_period, lifespan, precipitation, bloom_period, scientific_name, generic_name)
    
    return <div style={{height:"24rem", width:"20rem", border:"1px solid black", borderRadius: "15px", overflow:"hidden", margin:"0px 10px"}}>
        <section  style={{width:"100%"}}>
            <img src={img} style={{width:"100%", objectFit: "cover"}}></img>
        </section>
        <section style={{ display:"flex", justifyContent:"center", alignItems:"center", flexDirection:"column", width:"100%"}}>
            <h1 style={{margin:0}}>{scientific_name}</h1>
            <p style={{margin:0}}>{generic_name}</p>    
        <section style={{width:"100%"}}>
            <ul>
                <li>Active Growth Period: {active_growth_period}</li>
                <li>Lifespan: {lifespan}</li>
                <li>Preciptation: {precipitation}</li>
                <li>Bloom Period: {bloom_period}</li>
            </ul>
        </section>
        </section>
    
    </div>
}