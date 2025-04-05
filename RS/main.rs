use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream}; // Importar TcpListener y TcpStream de Tokio
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Importar traits para operaciones asíncronas
use database::Database;

// --> Asignacion de los modulos
mod database;
mod models;
mod routes;

// --> Funcion Main
#[tokio::main]
async fn main() {

    let logo: &str = r#"
   ____       _ _      _   _ _            
  / ___| __ _| | | ___| |_(_) |_ __ _ ___ 
 | |  _ / _` | | |/ _ \ __| | __/ _` / __|
 | |_| | (_| | | |  __/ |_| | || (_| \__ \
  \____|\__,_|_|_|\___|\__|_|\__\__,_|___/                                 
    "#;
    println!("-----------------------------------------");
    println!("{}", logo);
    println!("-----------------------------------------");

    // --> Conectar a la base de datos
    let db = Arc::new(Mutex::new(database::Database::connect().await.expect("Error al conectar a la base de datos")));
    
    // --> Iniciar el servidor
    let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap(); // Usar .await aquí
    println!("SERVIDOR DE GALLETAS INICIADO!!!\nVisite el puerto: http://127.0.0.1:3030\nAunque el CRUD funciona solo por POSTMAN");

    loop {
        let (stream, _) = listener.accept().await.unwrap(); // Usar .await aquí
        let db_clone = Arc::clone(&db);
        tokio::spawn(async move {
            handle_connection(stream, db_clone).await;
        });
    }

}

// --> Manejar la coneccion de los usuarios en la Web
async fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<Database>>) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap(); // Usar .await aquí

    let request = String::from_utf8_lossy(&buffer[..n]); 
    let response = {
        let db_lock = db.lock().await; // Usar el lock asíncrono
        routes::handle_request(request.to_string(), &*db_lock).await // Pasar el guard a la función
    }; 

    stream.write_all(response.as_bytes()).await.unwrap(); // Usar write_all para asegurarse de que se escriba todo
    // No es necesario llamar a flush aquí, ya que write_all maneja eso
}