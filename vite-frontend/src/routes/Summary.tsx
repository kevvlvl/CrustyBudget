import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {Box, Center, Grid, Loader, Paper, Table, type TableData} from "@mantine/core";
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

const COLOR_MAP: Record<string, string> = {
    Salary: 'orange.6',
    Investments: 'indigo.7',
    Other: 'cyan.5',
};

export default function Summary() {

    const [incomeTableData, setIncomeTableData] = useState<SummaryDataItem[]>([]);
    const [incomeGraphData, setIncomeGraphData] = useState<ChartDataItem[]>([]);
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
                        color: COLOR_MAP[rawData.items[i].category] || 'gray.4',
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
    }, []);

    const tableData: TableData = useMemo(() => ({
        head: ['Name', 'Category', 'Amount'],
        body: incomeTableData.map((item) => [
            item.name,
            item.category,
            `$${Number(item.amount).toFixed(2)}`,
        ]),
    }), [incomeTableData]);

    if (loading)
        return <Center h={350}><Loader color="blue" /></Center>;

    if (error)
        return <Center h={350}>Error: {error}</Center>;

    return(

        <Grid gutter="md" align="flex-start">
            <Grid.Col span={{ base: 12, md: 6 }}>

                <h2>Income</h2>

                <Paper withBorder p="md" radius="md" h="100%">

                <Box h={350} w={450}>

                    <PieChart
                        data={incomeGraphData}
                        labelsPosition={"outside"}
                        labelsType={"value"}
                        h={250}
                        size={250}
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

                <Table data={tableData} striped highlightOnHover verticalSpacing="sm" />

                </Paper>

            </Grid.Col>
        </Grid>
    );
}