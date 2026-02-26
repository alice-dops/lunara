import "./App.scss";
import { Lunara } from "./components/lunara";
import { LunaraInputProvider } from "./contexts/lunarInputContext";

function App() {

  return (
    // <React.StrictMode>
    <LunaraInputProvider>
      <Lunara />
    </LunaraInputProvider>
    // </React.StrictMode>
  );
}

export default App;
