use regex::Regex;
use regex::Replacer;
use std::io;
use std::fs;

fn main() {
    loop{
        let mut input_user = String::new();
        print!("Ingrese regular expresion o presione <salir> y <enter> para cerrar programa\n");
        io::stdin().read_line(&mut input_user).unwrap();
        input_user = input_user.trim().to_string();

        if input_user == "salir" { break; }

        let path_csv= "movies.csv";
        let mut contenido_csv = fs::read_to_string(path_csv).unwrap();
        let regular_e = Regex::new(&input_user).unwrap();

        loop {
            let cont_csv = contenido_csv.clone();
            let captura_regular_e = regular_e.captures(&cont_csv);

            if captura_regular_e.is_none(){break;}
            
            let captura = captura_regular_e.unwrap();
            print!("{:?}\n", captura);
            let c = captura.get(0).unwrap().as_str().to_string();
            contenido_csv.replace_range(cont_csv.find(&c)..c.len(), replace_with)

        }
    }
}
    
