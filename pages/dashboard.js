import Card from "../components/Card/Card"
import { useState, useEffect} from "react"
import Form from "../components/Form/Form"
import Navbar from "../components/Navbar/Navbar"
import Viewport from "../components/Viewport/Viewport"
export default function Dashboard() {
    const [data, setData] = useState({value:null})
    useEffect(()=>{
        // fetch("api/get_plant_info",{
        //     method: 'POST', 
        //     mode: 'cors', 
        //     cache: 'no-cache', 
        //     credentials: 'same-origin', 
        //     headers: {
        //       'Content-Type': 'application/json'
        //     },
        //     redirect: 'follow', 
        //     referrerPolicy: 'no-referrer', 
        //     body: JSON.stringify("ABBA") 
        //   }).then(response=>response.json()).then(data=>{
        //     setData(data)
        //   })
    })
    return <><Navbar/><Viewport><Card/></Viewport></>
}