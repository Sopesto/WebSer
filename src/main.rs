use hws::PiletaDeHilos;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pileta = PiletaDeHilos::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
				pileta.ejecutar(|| {
            manejar_conexion(stream);
        });
    }

    println!("Apagando.");
}

fn manejar_conexion(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    //let http_request: Vec<_> = buf_reader.lines().map(|resultado| resultado.unwrap()).take_while(|linea| !linea.is_empty()).collect();
    let linea_solicitud = buf_reader.lines().next().unwrap().unwrap();

    let (estado, nombre_archivo) = match &linea_solicitud[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contenido = fs::read_to_string(nombre_archivo).unwrap();
    let longitud = contenido.len();

    let respuesta = format!("{estado}\r\nContent-Length: {longitud}\r\n\r\n{contenido}");
    stream.write_all(respuesta.as_bytes()).unwrap();
    stream.flush().unwrap();
}
