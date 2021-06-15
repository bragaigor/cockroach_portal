#![allow(proc_macro_derive_resolution_fallback)]
#![allow(clippy::upper_case_acronyms)]
#[cfg(test)]
pub mod tests {
    use anyhow::Result;
    
    use crate::query::*;

    pub async fn create_testing_cockroachdb_env() -> (DBConn, DB) {
        println!("Inside create_testing_cockroachdb_env... About to connect to the database");
        let db = DB::new_cockroach_test().await.expect("could not connect");
        let dbc = db.conn().await.expect("ok");
    
        (dbc, db)
    }

    #[tokio::test]
    async fn test_transformers() -> Result<()> {
        println!("Inside test_transformers test...");
        let (mut dbc, _) = create_testing_cockroachdb_env().await;
        println!("Connection succesful!!");

        let transformer = TransformerInput {
            title: "Bumblebee".to_owned(),
            power: 1000
        };
        let transformer2 = TransformerInput {
            title: "Bumblebee".to_owned(),
            power: 3000
        };
        let transformer3 = TransformerInput {
            title: "Bumblebee".to_owned(),
            power: 5000
        };

        // If we try to create a transaction by uncommenting the below 2 comments
        // We get an error from CockroachDB "unimplemented: multiple active portals not supported"
        // >>>>>>>>>>>>>>>>
        // sqlx::query("BEGIN;").execute(&mut dbc).await?;

        let _result_id = add_transformer(&mut dbc, transformer).await?;
        let _result_id = add_transformer(&mut dbc, transformer2).await?;
        let _result_id = add_transformer(&mut dbc, transformer3).await?;

        let transformers = get_transformers(&mut dbc).await?;

        // sqlx::query("COMMIT;").execute(&mut dbc).await?;

        println!("Transformers: {:?}", transformers);

        Ok(())
    }
}