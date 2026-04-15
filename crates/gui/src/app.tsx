import "./index.css";
import { HttpPageContainer } from "./modules/http-page";

function App() {

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
      <HttpPageContainer />
    </main>
  );
}

export default App;
