import AddIncome from "./components/AddIncome.tsx";
import AddExpense from "./components/AddExpense.tsx";

import '@mantine/core/styles.css';

import {AppShell, Button, Group, MantineProvider, SimpleGrid, Stack} from '@mantine/core'
import AddCCEntry from "./components/AddCCEntry.tsx";

function App() {

    return (
      <MantineProvider defaultColorScheme="auto">
          <AppShell header={{height: 60}}>
              <AppShell.Header p={"md"}>
                  <Group gap={"lg"}>
                      <Button variant={"default"}>Show Income Details</Button>
                      <Button variant={"default"}>Show Expense Details</Button>
                      <Button variant={"default"}>Show Credit Details</Button>
                      <Button variant={"default"}>Generate Report</Button>
                  </Group>
              </AppShell.Header>

              <AppShell.Main>

                  <Stack align={"stretch"} justify={"center"} gap={"md"} style={{marginLeft: 20}}>

                      <SimpleGrid cols={2}>
                          <div>
                              <AddIncome />
                          </div>

                          <div>
                              <AddExpense />
                          </div>
                      </SimpleGrid>

                      <AddCCEntry />
                  </Stack>

              </AppShell.Main>
          </AppShell>
      </MantineProvider>
    );
}

export default App
