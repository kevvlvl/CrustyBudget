import {SimpleGrid, Stack} from "@mantine/core";
import AddIncome from "../components/AddIncome.tsx";
import AddExpense from "../components/AddExpense.tsx";
import AddCCEntry from "../components/AddCCEntry.tsx";

export default function EntryForms() {
    return (
        <div>
            <h2>Entry form</h2>
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
        </div>
    );
}