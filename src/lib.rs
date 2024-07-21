use std::{
  sync::{mpsc, Arc, Mutex},
  thread,
};

pub struct PiletaDeHilos {
  trabajadores: Vec<Trabajador>,
  emisor: Option<mpsc::Sender<Trabajo>>,
}

struct Trabajador {
  id: usize,
  hilo: Option<thread::JoinHandle<()>>,
}

type Trabajo = Box<dyn FnOnce() + Send + 'static>;

impl PiletaDeHilos {
  /// Crea una nueva PiletaDeHilos
  ///
  /// El tamaño es el número de hilos en la pileta.
  ///
  /// # Panics
  ///
  /// La función "new" entrará en pánico si el tamaño es cero o menos.
  pub fn new(size: usize) -> PiletaDeHilos {
    assert!(size > 0);

    let mut trabajadores = Vec::with_capacity(size);
    let (emisor, receptor) = mpsc::channel();
    let receptor = Arc::new(Mutex::new(receptor));

    for i in 0..size {
      trabajadores.push(Trabajador::new(i, Arc::clone(&receptor)));
    }
    PiletaDeHilos {
      trabajadores,
      emisor: Some(emisor),
    }
  }

  pub fn ejecutar<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let trabajo = Box::new(f);
    self.emisor.as_ref().unwrap().send(trabajo).unwrap();
  }
}

impl Drop for PiletaDeHilos {
  fn drop(&mut self) {
    drop(self.emisor.take());

    for trabajador in &mut self.trabajadores {
      println!("Apagando trabajador {}", trabajador.id);

      if let Some(hilo) = trabajador.hilo.take() {
        hilo.join().unwrap();
      }
    }
  }
}

impl Trabajador {
  fn new(id: usize, receptor: Arc<Mutex<mpsc::Receiver<Trabajo>>>) -> Trabajador {
    let hilo = thread::spawn(move || loop {
    let mensaje = receptor.lock().unwrap().recv();
      match mensaje {
        Ok(trabajo) => {
          println!("Trabajador {id} obtuvo un trabajo. \n ejecutando...");
          trabajo();
          println!("Trabajo terminado");
        }
        Err(_) => {
          println!("Trabajador {id} desconectado. \n apagando...");
          break;
        }
      }
    });

    Trabajador {
      id,
      hilo: Some(hilo),
    }
  }
}
