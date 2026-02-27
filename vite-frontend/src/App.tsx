import '@mantine/core/styles.css';

import {BrowserRouter, Routes, Route} from "react-router-dom";
import {MantineProvider} from '@mantine/core'
import {MainLayout} from "./layout/MainLayout.tsx";
import EntryForms from "./routes/EntryForms.tsx";
import CreditDetails from "./routes/CreditDetails.tsx";
import Summary from "./routes/Summary.tsx";

function App() {

    return (
      <MantineProvider defaultColorScheme="auto">
          <BrowserRouter>
              <Routes>
                  <Route path ="/" element={<MainLayout />}>
                      <Route index element={<EntryForms />} />
                      <Route path="summary" element={<Summary />} />
                      <Route path="creditdetails" element={<CreditDetails />} />
                  </Route>
              </Routes>
          </BrowserRouter>
      </MantineProvider>
    );
}

export default App
