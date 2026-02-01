use log::info;
use redb::{Database, Error, ReadableTable, TableDefinition, TableHandle, WriteTransaction};

const DB_NAME: &str = "crusty.redb";
const ID_SEQ: TableDefinition<&str, u64> = TableDefinition::new("id_seq");
pub const INCOME_TABLE: TableDefinition<u64, &str> = TableDefinition::new("income");
pub const EXPENSE_TABLE: TableDefinition<u64, &str> = TableDefinition::new("expenses");

pub fn save(payload: &str, table_definition: TableDefinition<u64, &str>) -> Result<(), Error> {

    let db = Database::create(DB_NAME)?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(table_definition)?;
        let id: u64 = get_and_increment_id(&write_txn, table_definition)?;

        table.insert(id, payload)?;
    }

    write_txn.commit()?;

    Ok(())
}

fn get_and_increment_id(write_txn: &WriteTransaction, table_definition: TableDefinition<u64, &str>) -> Result<u64, Error> {
    let mut table = write_txn.open_table(ID_SEQ)?;
    let current_id = table.get(table_definition.name())?.map(|v| v.value()).unwrap_or(0);
    let next_id = current_id + 1;

    info!("Next ID increment for table {0} is {1}", table_definition.name(), next_id);

    table.insert(table_definition.name(), next_id)?;
    Ok(next_id)
}