import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { getClient, Body, ResponseType } from "@tauri-apps/api/http";

const client = await getClient();
const login = await client.request({
  url: "http://localhost:8080/api/login",
  method: "POST",
  body: Body.json({
    username: "demo1",
    password: "welcome",
  }),
  responseType: ResponseType.JSON,
})

const payment = await client.request({
  url: "http://localhost:8080/api/payments",
  headers: {
    Cookie: "auth-token=user-1.exp.sign"
  },
  method: "POST",
  body: Body.json({
    amount: "100",
    sender: "demo1",
    receiver: "demo1",
    description: "allowance"
  })
})

const response = await client.request({
  url: "http://localhost:8080/api/payments",
  method: "GET",
  headers: {
    Cookie: "auth-token=user-1.exp.sign"
  },
});

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <pre>{JSON.stringify(login, undefined, 2)}</pre>
      <pre>{JSON.stringify(payment, undefined, 2)}</pre>
      <pre>{JSON.stringify(response, undefined, 2)}</pre>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
