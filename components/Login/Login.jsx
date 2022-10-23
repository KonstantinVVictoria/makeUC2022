import styles from "./Login.module.css"
import { useState } from "react"
import themes from "../../styles/themes"
export default function Login() {
    const [state, changeState] = useState({signUp: false})
    return <div className={styles.Login}>
            <div className={styles.Section1}>
                <img className={styles.Icon} src="logo.svg"/>
                <SignUpToggle signUp={state.signUp}/>
            </div>
            <div className={styles.Section2}>
                <Input field="email" label="Email"/>
                <Input field="user" label="Username"/>
                <Input field="pass" label="Password"/>
            </div>
            <div className={styles.Section3}>
                <Button/>
            </div>
        </div>
} 

function SignUpToggle({signUp}) {
    console.log
    return <div className={styles.Toggle}><h1 style={{margin:0, color: !signUp ? themes.colors.main: "black", cursor: signUp? "pointer":""}}>Sign Up</h1><h1 style={{margin:"0px 1rem", color: signUp ? themes.colors.main: "black"}}>|</h1><h1 style={{margin:0, color: signUp ? themes.colors.main: "black", cursor: !signUp? "pointer":""}}> Login</h1></div>
}

function Input({field, label}) {
    return <div style={{width:"80%"}}><h2 style={{margin:0}}>{label}</h2><input className={styles.Input} id={field}/></div>
}

function Button({signUp}){
    return <button className={styles.Button}>{!signUp?"Sign Up":"Login"}</button>
}