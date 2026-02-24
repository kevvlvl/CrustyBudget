import { PieChart } from '@mantine/charts';
import '@mantine/charts/styles.css'
import {Center, Container, Loader} from "@mantine/core";
import {useEffect, useState} from "react";

interface IncomeData {
    source: string;
    amount: number;
    frequency: string;
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

export default function IncomeDetails() {

    const [data, setData] = useState<ChartDataItem[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchIncomes = async () => {
            try {
                const response = await fetch('http://localhost:3000/api/budget/income');
                if (!response.ok)
                    throw new Error('Network response was not ok');

                const rawData: IncomeData[] = await response.json();

                console.log('rawData = ', rawData);

                // TRANSFORM: Map the raw Rust data to Chart-friendly format with colors
                const formattedData: ChartDataItem[] = rawData.map((item) => ({
                    name: item.source,
                    value: Number(item.amount),
                    // Fallback to gray if category isn't in our map
                    color: COLOR_MAP[item.source] || 'gray.4',
                }));

                console.log('formattedData = ', formattedData);

                setData(formattedData);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Unknown error');
            } finally {
                setLoading(false);
            }
        };

        fetchIncomes();
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
            <h2>Income details</h2>

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
        </Container>
    );
}