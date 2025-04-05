use crate::database::Database;
use crate::models::User ;
use serde_json::json;

// --> Manejador de solicitudes
pub async fn handle_request(request: String, db: &Database) -> String {

    // --> Verificacion que la request no este vacia
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return "HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_string();
    }

    // --> Verificacion que la request, para solicitud HTTP válida
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 3 {
        return "HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_string();
    }

    // --> Obtencion del metodo y la ruta
    let method = parts[0];
    let path = parts[1];

    // --> Verificacion del tipo de rutas
    match (method, path) {
        ("GET", "/users") => {
            let users = db.get_users().await.unwrap_or_else(|_| vec![]);
            let json = serde_json::to_string(&users).unwrap();
            format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", json)
        }
        
        ("POST", "/users") => {
            let (headers, body) = match request.split_once("\r\n\r\n") {
                Some((headers, body)) => (headers, body),
                None => {
                    eprintln!("Error: No se pudo separar los encabezados del cuerpo");
                    return format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nMalformed request");
                }
            };
            println!("Cuerpo JSON: {}", body);
            let new_user: User = serde_json::from_str(body).unwrap();
            let user = db.create_user(new_user.ci, &new_user.nombre, new_user.galletas).await.unwrap();
            let json = serde_json::to_string(&user).unwrap();
            format!("HTTP/1.1 201 CREATED\r\nContent-Type: application/json\r\n\r\n{}", json)
        }

        ("PUT", path) if path.starts_with("/users/") => {
            // Manejo seguro del `ci`
            let ci_result: Result<i32, _> = path.trim_start_matches("/users/").parse();
            let ci = match ci_result {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Error al parsear 'ci': {:?}", e);
                    return format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nInvalid user ID");
                }
            };

            // Manejo del cuerpo de la solicitud
            let (headers, body) = match request.split_once("\r\n\r\n") {
                Some((headers, body)) => (headers, body),
                None => {
                    eprintln!("Error: No se pudo separar los encabezados del cuerpo");
                    return format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nMalformed request");
                }
            };
            let updated_user_result: Result<User, serde_json::Error> = serde_json::from_str(body);
            let updated_user = match updated_user_result {
                Ok(user) => user,
                Err(e) => {
                    eprintln!("Error al deserializar JSON: {:?}", e);
                    return format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nInvalid JSON format");
                }
            };
        
            // Llamada al método update_user
            match db.update_user(ci, &updated_user.nombre, updated_user.galletas).await {
                Ok(user) => {
                    let json_result = serde_json::to_string(&user);
                    match json_result {
                        Ok(json) => format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", json),
                        Err(e) => {
                            eprintln!("Error al serializar respuesta JSON: {:?}", e);
                            format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nFailed to serialize response")
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error al actualizar usuario: {:?}", e);
                    format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nFailed to update user")
                }
            }
        }

        ("DELETE", path) if path.starts_with("/users/") => {
            let ci: i32 = path.trim_start_matches("/users/").parse().unwrap();
            db.delete_user(ci).await.unwrap();
            "HTTP/1.1 204 NO CONTENT\r\n\r\n".to_string()
        }
        
        // --> Predeterminado!
        _ => {
            let json = json!({"error": "Ruta no encontrada"}).to_string();
            format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n\r\n{}", json)
        }
    }
}