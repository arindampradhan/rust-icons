import { Routes, Route } from "react-router-dom";
import CollectionHome from "./pages/CollectionHome";
import CollectionHome2 from "./pages/CollectionHome2";

import CollectionDetail from "./pages/CollectionDetail";
import Daily from "./pages/Design_Daily";

function App() {
  return (
    <Routes>
      <Route path="daily" element={<Daily />} />
      <Route path="/" element={<CollectionHome />} />
      <Route path="/2" element={<CollectionHome2 />} />
      <Route path="/collection/:id" element={<CollectionDetail />} />
    </Routes>
  );
}

export default App;
