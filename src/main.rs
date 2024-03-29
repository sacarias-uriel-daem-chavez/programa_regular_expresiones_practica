use regex::Regex;
use std::io;
use std::fs;

fn main() {
    loop{
        let mut input_user = String::new();
        print!("\nIngrese su regular expresion o ingrese '***' y presione <enter> para cerrar programa\n");
        io::stdin().read_line(&mut input_user).unwrap();
        input_user = input_user.trim().to_string();
        print!("\n");

        if input_user == "***" { break; }

        //let path_csv= "movies.csv";
        //let path_liners = "liners_phone";
        let path_csv = "resultados_futbol.csv";
        let contenido_csv = fs::read_to_string(path_csv).unwrap();
        let regular_e = Regex::new(&input_user).unwrap_or(Regex::new(r".").unwrap());
        let clone_contenido_csv = contenido_csv.clone();
        let rows_de_contenido = clone_contenido_csv.lines();

        for line in rows_de_contenido.clone(){
            let mut manipulacion_line = line.to_string();
            let captura_regular_e = regular_e.captures(&manipulacion_line);
                
            if captura_regular_e.is_some(){
                print!("{}\n", line);
                loop{
                    let captura_regular_e = regular_e.captures(&manipulacion_line);

                    if captura_regular_e.is_none(){break;}

                    let captura = captura_regular_e.unwrap().get(0).unwrap().as_str();

                    if captura == "" {break;}

                    let replacen_line = manipulacion_line.replacen(captura, "", 1);
                    print!(">{} ", captura);
                    manipulacion_line = replacen_line;
                }
                print!("\n\n");
            }
        }

        print!("\n---------------------------------------------------------------------------------------------------------")
    }
}
    
