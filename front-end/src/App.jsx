import { useState } from 'react';
import './App.css';
import Login from './Login';

function App() {
    const [user, setUser] = useState(null)

    if(!user){
        return (
            <>
                <h1>Login</h1>
                <div className="card">
                    <Login user={user} setUser={setUser}/>
                </div>
            </>
        )
    } else{
        return (
            <>
                <h1>Welcome {user.username}</h1> 
                <h2>{user.uuid}</h2>
            </>
        )
    }
}

export default App
