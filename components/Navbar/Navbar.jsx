import styles from "./NavBar.module.css"
import nav_items from "./nav_items.json"
import themes from "../../styles/themes"
export default function NavBar() {
    return <div className={styles.NavBar}>
        <WebTitle/> <NavItems/>
    </div>
}


function WebTitle(){
    return <div className={styles.WebTitle}>
        <Title>{themes.title}</Title>
        <Icon image={"logo.svg"}/>
    </div>
}
function Title({children}){
    return <h1 className={styles.Title}>{children}</h1>
}

function Icon({image}){
    return <img className={styles.Icon} src={image}/>
}
function NavItems(){
    const NavItemsList = nav_items.map(({label, link}, i)=> <NavItem key={i+"_nav_items"}label={label} link={link}/>)
    return <div className={styles.NavItems}>{NavItemsList}</div>
}
function NavItem({label, link}){
    return <a className={styles.NavItem} href={link}>{label}</a>
}
