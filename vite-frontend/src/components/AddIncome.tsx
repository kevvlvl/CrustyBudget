import { useState } from 'react';
import { Button, TextInput, NativeSelect } from '@mantine/core';

const AddIncome = () => {
    const [formData, setFormData] = useState({
        source: '',
        amount: '',
        frequency: 'once'
    });
    const [status, setStatus] = useState({ type: '', message: '' });

    const handleSubmit = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        setStatus({ type: 'info', message: 'Sending...' });

        try {
            const response = await fetch('http://localhost:3000/api/budget/income', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    source: formData.source,
                    amount: parseFloat(formData.amount),
                    frequency: formData.frequency
                }),
            });

            if (response.ok) {
                setStatus({ type: 'success', message: 'Income added' });
                setFormData({ source: '', amount: '', frequency: 'Weekly' });
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
            <h2>Add Income</h2>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                <TextInput
                    placeholder="Source (e.g. Salary)"
                    value={formData.source}
                    onChange={(e) => setFormData({...formData, source: e.target.value})}
                    required
                />
                <TextInput
                    type="number"
                    placeholder="Amount"
                    value={formData.amount}
                    onChange={(e) => setFormData({...formData, amount: e.target.value})}
                    required
                />
                <NativeSelect
                    value={formData.frequency}
                    onChange={(e) => setFormData({...formData, frequency: e.target.value})}
                >
                    <option value="Daily">Daily</option>
                    <option value="Weekly">Weekly</option>
                    <option value="Biweekly">Biweekly</option>
                    <option value="Monthly">Monthly</option>
                </NativeSelect>
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

export default AddIncome;