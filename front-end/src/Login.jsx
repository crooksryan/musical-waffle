export default function Login({user, setUser}){

    async function signin(formdata){
        formdata.preventDefault()

        let username = document.getElementById('username').value;
        let uuid = Number(document.getElementById('uuid').value);

        let tempUser = {
            'username': username,
            'uuid': uuid
        };

        let temp = {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(tempUser)
        }
        try{
            let data = await fetch('/api/user', temp);
            let json = await data.json();

            if(json.username && json.uuid){
                setUser(json)
            }
        } catch (err){
            console.warn(err)
        }
    }

    return (
        <form onSubmit={signin}>
            <input id='username' placeholder="username" autoComplete="off"/>
            <input id='uuid' placeholder="uuid" autoComplete="off"/>
            <button>Login</button>
        </form>
    )
}
