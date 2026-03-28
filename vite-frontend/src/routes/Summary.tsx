import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {
    Container,
    Grid,
    Group,
    NativeSelect,
    Paper, Stack,
    Table,
    type TableData,
    Text
} from "@mantine/core";
import {IconClock} from '@tabler/icons-react';
import {useEffect, useMemo, useState} from "react";

interface Report {
    items: AggregatedItemDetails[];
}

interface AggregatedItemDetails {
    calculated_amount: number,
    frequency: string,
    item: Identifier,
}

interface Identifier {
    id: number,
    item_details: ItemDetails,
}

interface ItemDetails {
    amount: number,
    expense_category: string,
    expense_due_date: Date,
    frequency: string,
    income_category: string,
    name: string
}

interface ChartDataItem {
    name: string;
    value: number;
    color: string;
}

const PIE_CHAT_COLOR_MAP: Record<string, string> = {
    Salary: 'orange.6',
    Investments: 'indigo.7',
    Utilities: 'red.3',
    Electricity: 'blue.3',
    OnlineServices: 'green.5',
    Other: 'gray.5',
};

export default function Summary() {

    const [incomeTotal, setIncomeTotal] = useState(0);
    const [expenseTotal, setExpenseTotal] = useState(0);
    const [frequency, setFrequency] = useState('Monthly');
    const [incomeTableData, setIncomeTableData] = useState<AggregatedItemDetails[]>([]);
    const [graphData, setGraphData] = useState<ChartDataItem[]>([]);
    const [expenseTableData, setExpenseTableData] = useState<AggregatedItemDetails[]>([]);

    useEffect(() => {

        const fetchData = async() => {

            const incomesResponse = await fetch(`http://localhost:3000/api/budget/income?frequency=${frequency}`);
            const incomesData: Report = await incomesResponse.json();

            console.log('incomesData = ', incomesData);

            const expensesResponse = await fetch(`http://localhost:3000/api/budget/expense?frequency=${frequency}`);
            const expensesData: Report = await expensesResponse.json();

            console.log('expensesData = ', expensesData);

            const formattedChartData: ChartDataItem[] = [];

            for(let i = 0; i < incomesData.items.length; i++) {
                formattedChartData.push({
                    name: incomesData.items[i].item.item_details.name,
                    value: Number(incomesData.items[i].calculated_amount),
                    color: PIE_CHAT_COLOR_MAP[incomesData.items[i].item.item_details.income_category],
                });
            }

            setIncomeTableData(incomesData.items);
            setIncomeTotal(incomesData.items.reduce((acc, item) => acc + Number(item.calculated_amount), 0))

            for(let i = 0; i < expensesData.items.length; i++) {
                formattedChartData.push({
                    name: expensesData.items[i].item.item_details.name,
                    value: Number(expensesData.items[i].calculated_amount),
                    color: PIE_CHAT_COLOR_MAP[expensesData.items[i].item.item_details.expense_category],
                });
            }

            setExpenseTableData(expensesData.items);
            setExpenseTotal(expensesData.items.reduce((acc, item) => acc + Number(item.calculated_amount), 0))

            setGraphData(formattedChartData);
        }

        void fetchData();
    }, [frequency]);

    const incomeTableDataProp: TableData = useMemo(() => {

        return {
            head: ['Income', 'Category', 'Amount'],
            body: incomeTableData.map((item) => [
                item.item.item_details.name,
                item.item.item_details.income_category,
                `$${item.calculated_amount.toLocaleString(undefined, { minimumFractionDigits: 2 })}`,
            ]),
            foot: ['', 'Total', `$${incomeTotal.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`],
        }
    }, [incomeTableData, incomeTotal]);

    const expenseTableDataProp: TableData = useMemo(() => {

        return {
            head: ['Expense', 'Category', 'Amount'],
            body: expenseTableData.map((item) => [
                item.item.item_details.name,
                item.item.item_details.expense_category,
                `$${item.calculated_amount.toLocaleString(undefined, { minimumFractionDigits: 2 })}`,
            ]),
            foot: ['', 'Total', `$${expenseTotal.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`],
        }
    }, [expenseTableData, expenseTotal]);

    const spendingPower = useMemo(() => {
        return incomeTotal - expenseTotal
    }, [incomeTotal, expenseTotal]);

    return(
        <Container>

            <Grid gutter="md" align="flex-start">
                <Grid.Col span={{ base: 12, md: 6 }}>
                    <NativeSelect
                        w={200}
                        label={"Frequency"}
                        leftSection={<IconClock size={16} />}
                        withAsterisk
                        description={"All amounts will be recalculated for the set Frequency"}
                        data={['Daily', 'Weekly', 'Biweekly', 'Monthly']}
                        value={frequency}
                        onChange={(evt) => setFrequency(evt.currentTarget.value)}
                    />
                </Grid.Col>
                <Grid.Col span={{ base: 12, md: 6 }}>
                    <b>Budget</b>
                    <Group>
                        <Text c={"green"} fw={700}>Amount:</Text>
                        <Text>${spendingPower.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}</Text>
                    </Group>
                </Grid.Col>
            </Grid>

            <Stack>

                <PieChart
                    data={graphData}
                    labelsPosition={"outside"}
                    labelsType={"value"}
                    size={200}
                    withLabels
                    withTooltip

                />

                <Paper withBorder>
                    <Table data={incomeTableDataProp} highlightOnHover verticalSpacing="sm" />
                </Paper>

                <Paper withBorder>
                    <Table data={expenseTableDataProp} highlightOnHover verticalSpacing="sm" />
                </Paper>

            </Stack>

        </Container>
    );
}