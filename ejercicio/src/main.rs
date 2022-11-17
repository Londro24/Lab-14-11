use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;


#[derive(Default)]
#[derive(Debug)]
struct Partida{
    id: String,
    jx: String,
    jo: String,
    jugada: [[String; 3]; 3],
    ganador: String
}

fn is_entero_positivo(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else {
            return false
        }
    }
    return true
}
// Revisa si es un numero entero positivo

fn read_file(mut file: &File) -> String {
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}


fn create_blank_file(path: &Path){
    let _file = File::create(path).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn open_file(path: &Path) -> String{
    let mut text = "".to_string();
    if Path::new(path).exists(){
        let file = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(path);
    }
    return text
}


fn open_file_to_append(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}


fn print_tablero(partida: Partida) -> Partida {
    let mut tablero: [[&str; 3]; 3] = [[""; 3]; 3];
    print!("\n  | 1 | 2 | 3 | ");
    for a in 0..3 {
        match a {
            0 => print!("\n--+---+---+---+\na |"),
            1 => print!("\n--+---+---+---+\nb |"),
            _ => print!("\n--+---+---+---+\nc |")
        }
        for b in 0..3 {
            if partida.jugada[a][b] == "X" {
                print!(" X |")
            } else if partida.jugada[a][b] == "O" {
                print!(" O |")
            } else {
                print!("   |")
            }   
        }
    }
    return partida
}


fn crear_id(path: &Path) -> String{
    let mut contador = 0;
    let mut text = open_file(path);

    let mut id: String = String::new();
    println!("Ingrese el nombre de la partida");
    stdin().read_line(&mut id).unwrap();

    for a in text.split("\n") {
        for b in a.split(":") {
            if b == id.trim() ||  b == (id.trim().to_string() + &format!("({})", contador)){
                contador = contador + 1
            }
            break
        }
    }

    if contador != 0 {
        id = id.trim().to_string() + &format!("({})", contador)
    }
    println!("La partida ha sido guardada como {}", id);

    return id.trim().to_string()
}


fn pedir_jugador(numero: i8) -> String {
    let mut jugador: String = String::new();
    println!("Ingrese el nombre del jugador {}", numero);
    stdin().read_line(&mut jugador).unwrap();

    return jugador    
}


fn pedir_jugada(num: u32, mut partida: Partida) -> Partida {
    let mut fila = 0;
    let mut columna = 0;
    loop {
        let mut jugada: String = String::new();

        print!("turno del jugador {}:", num);
        stdin().read_line(&mut jugada).unwrap();

        let mut correcto = match &*jugada.to_lowercase().trim() {
            "a1" | "a2" | "a3" | "b1" | "b2" | "b3" | "c1" | "c2" | "c3" => true,
            _ => false
        };

        if jugada.to_lowercase().contains("a") {
            fila = 0; 
        } else if jugada.to_lowercase().contains("b") {
            fila = 1;
        } else {
            fila = 2;
        }

        if jugada.to_lowercase().contains("1") {
            columna = 0; 
        } else if jugada.to_lowercase().contains("2") {
            columna = 1;
        } else {
            columna = 2;
        }

        for a in 0..3 {
            for b in 0..3 {
                if a == fila && b == columna && partida.jugada[a][b] != "".to_string(){
                    correcto = false
                }
                
            }
        }

        if correcto {
            break
        }
    }
    if num == 1 {
        partida.jugada[fila][columna] = "X".to_string()
    } else {
        partida.jugada[fila][columna] = "O".to_string()
    }

    return partida
}


fn verificar_fin(mut partida: Partida) -> Partida {
    for a in 0..3 {
        if partida.jugada[0][a] == partida.jugada[1][a] && partida.jugada[2][a] == partida.jugada[1][a] {
            partida.ganador = partida.jugada[0][a].to_string()

        } else if partida.jugada[a][0] == partida.jugada[a][1] && partida.jugada[a][2] == partida.jugada[a][1] {
            partida.ganador = partida.jugada[a][0].to_string()
        }
    }
    if partida.jugada[0][0] == partida.jugada[1][1] && partida.jugada[2][2] == partida.jugada[1][1] {
        partida.ganador = partida.jugada[0][0].to_string()
    } else if partida.jugada[0][2] == partida.jugada[1][1] && partida.jugada[2][0] == partida.jugada[1][1] {
        partida.ganador = partida.jugada[0][2].to_string()
    }
    let mut contador = 0;
    for a in 0..3 {
        for b in 0..3 {
            if partida.jugada[a][b] != "".to_string() {
                contador += 1 
            }

        }
    }
    if contador == 9 {
        partida.ganador = "Empate".to_string()
    }
    return partida;
}


fn jugar_partida(path: &Path) {
    let mut partida: Partida = Default::default();
    partida.jx = pedir_jugador(1).trim().to_string();
    partida.jo = pedir_jugador(2).trim().to_string();
    loop {
        for a in 1..3 {
            partida = print_tablero(partida);
            partida = pedir_jugada(a, partida);
            partida = verificar_fin(partida);
            if partida.ganador != "".to_string() {
                break
            }
        }
    }

    partida.id = crear_id(path);

    println!("{:?}", partida)

}


fn consultar_partida(path: &Path) {


}


fn menu() -> u32 {
    let mut entrada: String = String::new();
    loop {
        println!("Elija opción:");
        println!("    (1) Jugar una nueva partida.");
        println!("    (2) Consultar una partida partida.");
        println!("    (0) Salir.");
        stdin().read_line(&mut entrada).unwrap();
        //
        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() {
            entrada = "".to_string();
            continue
        }
        //
        match entrada.trim().parse().unwrap() {
            0|1|2 => break,
            _ => entrada = "".to_string()
        }
        println!("\nIntentelo denuevo\n");
        continue
    }   
    let num: u32 = entrada.trim().parse().unwrap();
    return num
}


fn main() {
    let path: &Path = Path::new("Historial_de_partidas.txt");
    //
    loop {
        let opcion = menu();
        match opcion {
            1 => jugar_partida(path),
            2 => consultar_partida(path),
            _ => break
        }
    }
    
}
