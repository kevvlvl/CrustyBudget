# Crusty Budget App

Axum+Tokio API for a small budget/personal finance app. Exposes a few basic endpoints to handle personal financing needs

| API     | Description                                                                                                                                                                                                             |
|:--------|:------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Income  | Enter net amount and frequency within calendar month<br/>Get net amount left over after obligations<br/>Enter other income amount and frequency type (weekly, biweekly, monthly)                                        |
| Expense | Enter net amounts of monthly obligations. Frequenty 1 unless overriden otherwise (with frequency type: weekly, biweekly, monthly)<br/>Add CC balance: Enter date, amount, date due<br/>Add spending: Enter date, amount |

## Run the api

```
RUST_LOG=info cargo run
```

## Examples

### Add an income
```
curl -X POST localhost:3000/api/budget/income -H 'Content-Type: application/json' -d '{"source": "Work", "amount": "1500", "frequency": "Biweekly"}'
```

## Add an expense
```
curl -X POST localhost:3000/api/budget/expense -H 'Content-Type: application/json' -d '{"destination": "Cellphone Service", "amount": "45", "frequency": "Monthly"}'
```