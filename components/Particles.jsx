import "particles.js"
import { useEffect } from "react"


export default function Particles(){
    useEffect(()=>{
        particlesJS.load('particles-js', '/particles.json', function() {
            console.log('callback - particles.js config loaded');
          });
    })
    return<></>
}