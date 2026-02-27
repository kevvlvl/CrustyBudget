import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {Center, Container, Loader} from "@mantine/core";
import {useEffect, useState} from "react";

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

    const [data, setData] = useState<ChartDataItem[]>([]);
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

                return formattedChartData;
            })
            .then(result => {
                setData(result);
            })
            .catch(error => {
                console.error("Failed fetching: ", error);
                setError(error);
            })
            .finally(() => {
                setLoading(false);
            })
    }, []);

    if (loading)
        return <Center h={350}><Loader color="blue" /></Center>;

    if (error)
        return <Center h={350}>Error: {error}</Center>;

    return(
        <Container
            size={"responsive"}
            style={{maxWidth: '100%', maxHeight: '100%', width: 450, height: 400}}
        >
            <h2>Summary</h2>

            <h3>Incomes</h3>

            <PieChart
                data={data}
                labelsPosition={"outside"}
                labelsType={"value"}
                h={350}
                size={350}
                withLabels
                withLabelsLine
                withTooltip
            />

            <h3>Breakdown</h3>

            ITEMIZED TABLE HERE

            <h3>Expenses</h3>

            CHART HERE

            <h3>Breakdown</h3>

            ITEMIZED TABLE HERE

        </Container>
    );
}