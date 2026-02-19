import '@mantine/core/styles.css';

import {BrowserRouter, Routes, Route} from "react-router-dom";
import {MantineProvider} from '@mantine/core'
import {MainLayout} from "./layout/MainLayout.tsx";
import EntryForms from "./routes/EntryForms.tsx";
import IncomeDetails from "./routes/IncomeDetails.tsx";
import ExpenseDetails from "./routes/ExpenseDetails.tsx";
import CreditDetails from "./routes/CreditDetails.tsx";

function App() {

    return (
      <MantineProvider defaultColorScheme="auto">
          <BrowserRouter>
              <Routes>
                  <Route path ="/" element={<MainLayout />}>
                      <Route index element={<EntryForms />} />
                      <Route path="incomedetails" element={<IncomeDetails />} />
                      <Route path="expensedetails" element={<ExpenseDetails />} />
                      <Route path="creditdetails" element={<CreditDetails />} />
                  </Route>
              </Routes>
          </BrowserRouter>
      </MantineProvider>
    );
}

export default App
