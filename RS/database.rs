use tokio_postgres::{Client, NoTls, Error};
use crate::models;

pub struct Database {
    client: Client,
}

impl Database {
    // --> Conexion a la base de datos!
    pub async fn connect() -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=postgres password=admin dbname=Galletas",
            NoTls,
        ).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error en la conexión a la BD: {}", e);
            }
        });

        Ok(Self { client })
    }

    // --> Query GET
    pub async fn get_users(&self) -> Result<Vec<models::User >, Error> {
        let rows = self.client.query("SELECT ci, nombre, galletas FROM users", &[]).await?;
        let users = rows.iter().map(|row| models::User  {
            ci: row.get(0),
            nombre: row.get(1),
            galletas: row.get(2),
        }).collect();
        Ok(users)
    }

    // --> Query CREATE
    pub async fn create_user(&self, ci: i32, nombre: &str, galletas: i32) -> Result<models::User , Error> {
        let row = self.client.query_one(
            "INSERT INTO users(ci, nombre, galletas) VALUES ($1, $2, $3) RETURNING ci, nombre, galletas",
            &[&ci, &nombre, &galletas],
        ).await?;
        Ok(models::User  {
            ci: row.get(0),
            nombre: row.get(1),
            galletas: row.get(2),
        })
    }

    // --> Query UPDATE
    pub async fn update_user(&self, ci: i32, nombre: &str, galletas: i32) -> Result<models::User , Error> {
        let row = self.client.query_one(
            "UPDATE users SET nombre = $1, galletas = $2 WHERE ci = $3 RETURNING ci, nombre, galletas",
            &[&nombre, &galletas, &ci],
        ).await?;
        Ok(models::User  {
            ci: row.get(0),
            nombre: row.get(1),
            galletas: row.get(2),
        })
    }

    // --> Query DELETE
    pub async fn delete_user(&self, ci: i32) -> Result<u64, Error> {
        let rows_affected = self.client.execute(
            "DELETE FROM users WHERE ci = $1",
            &[&ci],
        ).await?;
        Ok(rows_affected)
    }
}