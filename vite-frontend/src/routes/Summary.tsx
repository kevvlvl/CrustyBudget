import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {Box, Center, Container, Grid, Loader, Paper, Table, type TableData} from "@mantine/core";
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

    const [incomeTableData, setIncomeTableData] = useState<SummaryDataItem[]>([]);
    const [incomeGraphData, setIncomeGraphData] = useState<ChartDataItem[]>([]);
    const [expenseTableData, setExpenseTableData] = useState<SummaryDataItem[]>([]);
    const [expenseGraphData, setExpenseGraphData] = useState<ChartDataItem[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetch('http://localhost:3000/api/budget/income?frequency=Monthly')
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

        fetch('http://localhost:3000/api/budget/expense?frequency=Monthly')
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
    }, []);

    const incomeTableDataProp: TableData = useMemo(() => ({
        head: ['Name', 'Category', 'Amount'],
        body: incomeTableData.map((item) => [
            item.name,
            item.category,
            `$${Number(item.amount).toFixed(2)}`,
        ]),
    }), [incomeTableData]);

    const expenseTableDataProp: TableData = useMemo(() => ({
        head: ['Name', 'Category', 'Amount'],
        body: expenseTableData.map((item) => [
            item.name,
            item.category,
            `$${Number(item.amount).toFixed(2)}`,
        ]),
    }), [expenseTableData]);

    if (loading)
        return <Center h={350}><Loader color="blue" /></Center>;

    if (error)
        return <Center h={350}>Error: {error}</Center>;

    return(
        <Container>
            <Grid gutter="md" align="flex-start">
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Income</h2>

                    <Paper withBorder p="md" radius="md" h="100%">
                        <Box h={280} w={300}>
                            <PieChart
                                data={incomeGraphData}
                                labelsPosition={"outside"}
                                labelsType={"value"}
                                h={200}
                                size={200}
                                withLabels
                                withLabelsLine
                                withTooltip
                            />
                        </Box>
                    </Paper>

                </Grid.Col>

                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Breakdown</h2>

                    <Paper withBorder p="md" radius="md" h="100%">
                        <Table data={incomeTableDataProp} striped highlightOnHover verticalSpacing="sm" />
                    </Paper>

                </Grid.Col>
            </Grid>
            <Grid>
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Expenses</h2>

                    <Paper withBorder p="md" radius="md" h="100%">
                        <Box h={200} w={200}>
                            <PieChart
                                data={expenseGraphData}
                                labelsPosition={"outside"}
                                labelsType={"value"}
                                h={200}
                                size={200}
                                withLabels
                                withLabelsLine
                                withTooltip
                            />
                        </Box>
                    </Paper>

                </Grid.Col>
                <Grid.Col span={{ base: 12, md: 6 }}>

                    <h2>Breakdown</h2>

                    <Paper withBorder p="md" radius="md" h="100%">
                        <Table data={expenseTableDataProp} striped highlightOnHover verticalSpacing="sm" />
                    </Paper>

                </Grid.Col>
            </Grid>
        </Container>
    );
}