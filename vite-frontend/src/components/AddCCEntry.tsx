import { useState } from 'react';
import { Button, NumberInput } from '@mantine/core';
import {DatePickerInput, type DatePickerValue} from "@mantine/dates";
import dayjs from "dayjs";

const AddCCEntry = () => {
    const [amount, setAmount] = useState<string | number>();
    const [paymentDate, setPaymentDate] = useState<DatePickerValue | undefined>();
    const [status, setStatus] = useState({ type: '', message: '' });

    const handleSubmit = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        setStatus({ type: 'info', message: 'Sending...' });

        try {
            const response = await fetch('http://localhost:3000/api/budget/cc', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    amount: amount,
                    payment_date: paymentDate?.toString(),
                }),
            });

            if (response.ok) {
                setStatus({ type: 'success', message: 'CC Entry added' });
                setAmount(0);
                setPaymentDate(dayjs().format());
            } else {
                setStatus({ type: 'error', message: `Server error: ${response.status}` });
            }
        } catch (e) {
            const errorMsg: string = 'Could not connect to Rust API';
            console.error(errorMsg, e);
            setStatus({ type: 'error', message: errorMsg });
        }
    };

    return (
        <div>
            <h2>Add Credit Card Entry</h2>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                <NumberInput
                    placeholder="Amount due"
                    value={amount}
                    onChange={setAmount}
                    required
                />
                <DatePickerInput
                    label={"Select Payment due date"}
                    placeholder={"Select Payment due date"}
                    value={paymentDate}
                    onChange={setPaymentDate}
                    />

                <Button type="submit">Submit</Button>
            </form>

            {status.message && (
                <p>
                    {status.message}
                </p>
            )}
        </div>
    );
};

export default AddCCEntry;