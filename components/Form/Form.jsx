import { getAuth } from "firebase/auth";
import PlantsList from "../plants_database.json"
let PlantsDatabase = {}
PlantsList.forEach(plant=> PlantsDatabase[plant["Common Name"].toLowerCase()]=plant)

export default function Form({setPlants, plants}){
    const auth = getAuth();
    const user = auth.currentUser;

    return<div style={{display:"flex", alignItems:"center", justifyContent:"center", flexDirection:"column"}}>
        <section style={{display:"flex", alignItems:"center", justifyContent:"center", flexDirection:"column"}}>
        <h1>Add Plant</h1>
        <input id="plant_name" style={{width:"250px", height:"2rem", margin:"0px 10px", fontWeight:"800", fontSize:"1.2rem", border:"none", borderBottom:"1px solid black"}}/>
        </section>
        <section style={{position:"relative", width:"100%"}}>
            <button onClick={()=>{
                const plant_name = document.getElementById("plant_name").value
                const plant = PlantsDatabase[plant_name.toLowerCase()]
                const newData = Object.values(plants)
                fetch("api/get_plant_info",{
                    method: 'POST', 
                    mode: 'cors', 
                    cache: 'no-cache', 
                    credentials: 'same-origin', 
                    headers: {
                    'Content-Type': 'application/json'
                    },
                    redirect: 'follow', 
                    referrerPolicy: 'no-referrer', 
                    body: JSON.stringify(newData) 
                }).then(response=>response.json()).then(([data])=>{
                    
                    console.log([{...plant,...data}])
                    setPlants([{...plant,...data}])
                })
        
            }} style={{position:"absolute", right:"0px", cursor:"pointer", backgroundColor:"#63FF6B", display:"flex", margin: "10px", fontSize:"2.4rem", borderRadius:"50px", height:"35px", width:"35px",alignItems:"center", justifyContent:"center"}}>+</button>
        </section>
        
    </div>
}