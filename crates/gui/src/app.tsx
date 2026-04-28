import { SplitPane } from "./components/split-pane";
import { SideMenu } from "./layout/side-menu";
import { HttpPageContainer } from "./modules/http-page";

function App() {

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
      <SplitPane>
        <SideMenu />
        <HttpPageContainer />
      </SplitPane>
    </main>
  );
}

export default App;
