import styles from "./Viewport.module.css"

export default function Viewport({children}){
    return <div className={styles.Viewport}>{children}</div>;
}