import { OpenCollectionButton } from "./components/open-collection-button";
import "./index.css";

function App() {

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
      <OpenCollectionButton />
    </main>
  );
}

export default App;
