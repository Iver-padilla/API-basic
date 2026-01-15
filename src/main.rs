use tiny_http::{Server, Response};

fn main() {
    
    let server = Server::http("0.0.0.0:3000").unwrap();
    println!("API corriendo en http://localhost:3000");

    
    for request in server.incoming_requests() {

        let url = request.url();

        
        if url == "/" {
            let response = Response::from_string("API funcionando");
            request.respond(response).unwrap();
        }

    
        else if url == "/hola" {
            let response = Response::from_string("Hola");
            request.respond(response).unwrap();
        }

        
        else if url.starts_with("/suma/") {
            let partes: Vec<&str> = url.split("/").collect();

            if partes.len() == 4 {
                let a: i32 = partes[2].parse().unwrap_or(0);
                let b: i32 = partes[3].parse().unwrap_or(0);
                let resultado = a + b;

                let response = Response::from_string(resultado.to_string());
                request.respond(response).unwrap();
            } else {
                let response = Response::from_string("Formato incorrecto");
                request.respond(response).unwrap();
            }
        }

        
        else {
            let response = Response::from_string("Ruta no encontrada");
            request.respond(response).unwrap();
        }
    }
}
