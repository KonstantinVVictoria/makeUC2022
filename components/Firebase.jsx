// Import the functions you need from the SDKs you need

import {initializeApp} from "firebase/app";
import { getAuth, signInWithEmailAndPassword } from "firebase/auth";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
// Initialize Firebase

export default function FirebaseContext(){
    const firebaseConfig = {
        apiKey: "AIzaSyBMTrueZxrsSXYQ1sQbeuHz4zcyUGiHWEI",
        authDomain: "arcata-9496f.firebaseapp.com",
        projectId: "arcata-9496f",
        storageBucket: "arcata-9496f.appspot.com",
        messagingSenderId: "156837359406",
        appId: "1:156837359406:web:c7c0bc82243f377d4b54bf",
        measurementId: "G-6XXPR9CW8S"
      };
      

        initializeApp(firebaseConfig)

      return <></>
}