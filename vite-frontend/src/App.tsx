import AddIncome from "./components/AddIncome.tsx";
import AddExpense from "./components/AddExpense.tsx";

import '@mantine/core/styles.css';

import {AppShell, Button, Collapse, Group, SimpleGrid, MantineProvider} from '@mantine/core'
import {useDisclosure} from "@mantine/hooks";
import AddCCEntry from "./components/AddCCEntry.tsx";

function App() {

    const [openedIncome, incomeHandlers] = useDisclosure(false);
    const [openedExpense, expenseHandlers] = useDisclosure(false);
    const [openedCCEntry, ccEntryHandlers] = useDisclosure(false);

    return (
      <MantineProvider defaultColorScheme="auto">
          <AppShell header={{height: 60}}>
              <AppShell.Header p={"md"}>
                  <Group gap={"lg"}>
                      <Button variant={"default"} onClick={incomeHandlers.toggle}>Add Income</Button>
                      <Button variant={"default"} onClick={expenseHandlers.toggle}>Add Expense</Button>
                      <Button variant={"default"} onClick={ccEntryHandlers.toggle}>Add Credit Card Entry</Button>
                      <Button variant={"default"}>Generate Report</Button>
                  </Group>
              </AppShell.Header>

              <AppShell.Main>

                  <SimpleGrid cols={3}>
                      <div>
                          <Collapse in={openedIncome} transitionTimingFunction={"linear"}>
                              <AddIncome />
                          </Collapse>
                      </div>
                      <div>
                          <Collapse in={openedExpense}>
                              <AddExpense />
                          </Collapse>
                      </div>
                      <div>
                          <Collapse in={openedCCEntry}>
                              <AddCCEntry />
                          </Collapse>
                      </div>
                  </SimpleGrid>

              </AppShell.Main>
          </AppShell>
      </MantineProvider>
    );
}

export default App
