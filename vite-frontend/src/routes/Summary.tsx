import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {Box, Center, Container, Grid, Loader, NativeSelect, Paper, Table, type TableData} from "@mantine/core";
import {IconClock} from '@tabler/icons-react';
import {useEffect, useMemo, useState} from "react";

interface SummaryData {
    frequency: string;
    items: SummaryDataItem[];
}

interface SummaryDataItem {
    amount: number;
    category: string,
    name: string,
}

interface ChartDataItem {
    name: string;
    value: number;
    color: string;
}

const INCOME_COLOR_MAP: Record<string, string> = {
    Salary: 'orange.6',
    Investments: 'indigo.7',
    Other: 'cyan.5',
};

const EXPENSE_COLOR_MAP: Record<string, string> = {
    Utilities: 'red.3',
    Electricity: 'blue.3',
    Services: 'green.5',
    Other: 'cyan.5',
};

export default function Summary() {

    const [frequency, setFrequency] = useState('Monthly');
    const [incomeTableData, setIncomeTableData] = useState<SummaryDataItem[]>([]);
    const [incomeGraphData, setIncomeGraphData] = useState<ChartDataItem[]>([]);
    const [expenseTableData, setExpenseTableData] = useState<SummaryDataItem[]>([]);
    const [expenseGraphData, setExpenseGraphData] = useState<ChartDataItem[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetch(`http://localhost:3000/api/budget/income?frequency=${frequency}`)
            .then(res => res.json())
            .then(res => {

                const rawData: SummaryData = res;

                console.log('rawData = ', rawData);

                const formattedChartData: ChartDataItem[] = [];

                for(let i = 0; i < rawData.items.length; i++) {
                    formattedChartData.push({
                        name: rawData.items[i].category,
                        value: Number(rawData.items[i].amount),
                        // Fallback to gray if category isn't in our map
                        color: INCOME_COLOR_MAP[rawData.items[i].category] || 'gray.4',
                    });
                }

                console.log('formattedData = ', formattedChartData);
                setIncomeTableData(rawData.items);
                return formattedChartData;
            })
            .then(result => {
                setIncomeGraphData(result);
            })
            .catch(error => {
                console.error("Failed fetching: ", error);
                setError(error);
            })
            .finally(() => {
                setLoading(false);
            })

        fetch(`http://localhost:3000/api/budget/expense?frequency=${frequency}`)
            .then(res => res.json())
            .then(res => {

                const rawData: SummaryData = res;

                console.log('rawData = ', rawData);

                const formattedChartData: ChartDataItem[] = [];

                for(let i = 0; i < rawData.items.length; i++) {
                    formattedChartData.push({
                        name: rawData.items[i].category,
                        value: Number(rawData.items[i].amount),
                        // Fallback to gray if category isn't in our map
                        color: EXPENSE_COLOR_MAP[rawData.items[i].category] || 'gray.4',
                    });
                }

                console.log('formattedData = ', formattedChartData);
                setExpenseTableData(rawData.items);
                return formattedChartData;
            })
            .then(result => {
                setExpenseGraphData(result);
            })
            .catch(error => {
                console.error("Failed fetching: ", error);
                setError(error);
            })
            .finally(() => {
                setLoading(false);
            })
    }, [frequency]);

    const incomeTableDataProp: TableData = useMemo(() => {

        const total = incomeTableData.reduce((acc, item) => acc + Number(item.amount), 0);

        return {
            head: ['Name', 'Category', 'Amount'],
            body: incomeTableData.map((item) => [
                item.name,
                item.category,
                `$${Number(item.amount).toFixed(2)}`,
            ]),
            foot: ['', 'Total', `$${Number(total).toFixed(2)}`],
        }
    }, [incomeTableData]);

    const expenseTableDataProp: TableData = useMemo(() => {

        const total = expenseTableData.reduce((acc, item) => acc + Number(item.amount), 0);

        return {
            head: ['Name', 'Category', 'Amount'],
            body: expenseTableData.map((item) => [
                item.name,
                item.category,
                `$${Number(item.amount).toFixed(2)}`,
            ]),
            foot: ['', 'Total', `$${Number(total).toFixed(2)}`],
        }
    }, [expenseTableData]);

    if (loading)
        return <Center h={350}><Loader color="blue" /></Center>;

    if (error)
        return <Center h={350}>Error: {error}</Center>;

    return(
        <Container>

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

            <Grid gutter="md" align="flex-start">
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Income</h2>
                    <Box h={280} w={350}>
                        <PieChart
                            data={incomeGraphData}
                            labelsPosition={"outside"}
                            labelsType={"percent"}
                            size={200}
                            withLabels
                            withTooltip
                        />
                    </Box>

                </Grid.Col>

                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Breakdown</h2>
                    <Paper withBorder>
                        <Table data={incomeTableDataProp} highlightOnHover verticalSpacing="sm" horizontalSpacing="md" />
                    </Paper>

                </Grid.Col>
            </Grid>
            <Grid>
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Expenses</h2>
                    <Box h={200} w={200}>
                        <PieChart
                            data={expenseGraphData}
                            labelsPosition={"outside"}
                            labelsType={"percent"}
                            size={200}
                            withLabels
                            withTooltip
                        />
                    </Box>

                </Grid.Col>
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Breakdown</h2>
                    <Paper withBorder p="md" radius="md" h="100%">
                        <Table data={expenseTableDataProp} highlightOnHover verticalSpacing="sm" />
                    </Paper>

                </Grid.Col>
            </Grid>
        </Container>
    );
}