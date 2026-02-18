use log::{ info};
use redb::{Database, DatabaseError, Error, ReadableDatabase, ReadableTable, TableDefinition, TableHandle, WriteTransaction};
use serde::de::DeserializeOwned;

const DB_NAME: &str = "crusty.redb";
const ID_SEQ: TableDefinition<&str, u64> = TableDefinition::new("id_seq");
pub const INCOME_TABLE: TableDefinition<u64, &str> = TableDefinition::new("income");
pub const EXPENSE_TABLE: TableDefinition<u64, &str> = TableDefinition::new("expenses");
pub const CC_TABLE: TableDefinition<u64, &str> = TableDefinition::new("cc");

fn get_and_increment_id(write_txn: &WriteTransaction, table_definition: TableDefinition<u64, &str>) -> Result<u64, Error> {
    let mut table = write_txn.open_table(ID_SEQ)?;
    let current_id = table.get(table_definition.name())?.map(|v| v.value()).unwrap_or(0);
    let next_id = current_id + 1;

    info!("Next ID increment for table {0} is {1}", table_definition.name(), next_id);

    table.insert(table_definition.name(), next_id)?;
    Ok(next_id)
}

fn get_db() -> redb::Result<Database, DatabaseError> {
    Database::create(DB_NAME)
}

pub fn save(payload: &str, table_definition: TableDefinition<u64, &str>) -> Result<(), Error> {

    let db = get_db()?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(table_definition)?;
        let id: u64 = get_and_increment_id(&write_txn, table_definition)?;

        table.insert(id, payload)?;
    }

    write_txn.commit()?;

    Ok(())
}

pub fn get<F, T>(value_filter: F, table_definition: TableDefinition<u64, &str>) -> Result<Vec<T>, String>
where
    F: Fn(&T) -> bool,
    T: DeserializeOwned,
{
    let db = get_db().map_err(|e| e.to_string())?;
    let read_txn = db.begin_read().map_err(|e| e.to_string())?;
    let table = read_txn.open_table(table_definition).map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for item in table.iter().map_err(|e| e.to_string())? {

        let(_key_access, value_access) = item.map_err(|e| format!("Storage error: {}", e))?;
        let json_str = value_access.value();

        let item_data : T = serde_json::from_str(json_str)
            .map_err(|e| e.to_string())?;

        if value_filter(&item_data) {
            results.push(item_data);
        }
    }

    Ok(results)
}