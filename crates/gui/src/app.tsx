import { SplitPane } from "./components/split-pane";
import { SideMenu } from "./layout/side-menu";
import { Container } from "./modules/container";
import { HttpPageContainer } from "./modules/http-page";
import { AppState } from "./store/app-store";

function App() {

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <AppState>
      <main className="container">
        <SplitPane>
          <SideMenu />
          <Container />
          {/* <HttpPageContainer /> */}
        </SplitPane>
      </main>
    </AppState>
  );
}

export default App;
