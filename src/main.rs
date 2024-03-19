
#[derive(Debug, Clone)]
struct Vector{
    header: String,
    col: Vec<String>,

}

const PATH: &str = "src/titanic.csv";
const VERBOSE: bool = true;

fn main() {
    let mut headers: Vec<String> = Vec::new();
    let mut vectores: Vec<Vector> = Vec::new();
    read_csv_vector(&mut vectores, &mut headers, PATH);
    
    //Obtenemos la combinatoria de las columnas:
    let combinaciones: usize= (1..=headers.len()).product();
    println!("Arboles: {}", combinaciones); //To-delete
    

    //VERSIÓN DINÁMICA:
    println!("\n########################----VERSIÓN DINÁMICA----########################");
    let mut contador_arboles = 0;
    let n_headers = headers.len();
    let mut ramas: Vec<String> = Vec::new();
    let mut ramas_datos: Vec<Vector> = Vec::new();
    let mut ramas_combinaciones: Vec<Vec<String>> = Vec::new();

    arbol_recursivo(n_headers, 
        headers, //HEADERS
        &mut contador_arboles, 
        0, 
        n_headers, 
        vectores.clone(), //DATOS
        &mut ramas, //VACIO
        &mut ramas_datos, //VACIO
        &mut ramas_combinaciones //VACIO
    );
}


fn arbol_recursivo(n_headers: usize,
    headers: Vec<String>, 
    contador_arboles: &mut usize, 
    depth: usize, 
    n_headers_original: usize, 
    vectores: Vec<Vector>,
    ramas: &mut Vec<String>,
    ramas_datos: &mut Vec<Vector>,
    ramas_combinaciones: &mut Vec<Vec<String>>
) {
    if n_headers != 0 {
        for i_header in 0..headers.len(){
            //GUARDAMOS LA RAMA ACTUAL EN EL VECTOR DE RAMAS:
            //IMPRIMIMOS LA RAMA ACTUAL:
            if n_headers == n_headers_original{  //ESTE SIGNIFICA ARBOL NUEVO
                ramas.clear();
                ramas_datos.clear();
                ramas_combinaciones.clear();
            }
            else if n_headers == 1 { //RAMA FINAL
                *contador_arboles += 1;
                while ramas.len() > depth{
                    ramas.pop();
                    ramas_datos.pop();
                    ramas_combinaciones.pop();
                }
            }
            
            else{ //RAMA INTERMEDIA
                while ramas.len() > depth{
                    ramas.pop();
                    ramas_datos.pop();
                    ramas_combinaciones.pop();
                }
            }

            ramas.push(headers[i_header].clone());
            ramas_datos.push(vectores[i_header].clone());
            
            let str_ramas = ramas.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" » ");
            if VERBOSE{
                println!("{}──Columna: {}", "     ".repeat(depth), str_ramas);
            }
            
            let mut valores_unicos: Vec<String> = ramas_datos.last().unwrap().col.clone();
            valores_unicos.sort();
            valores_unicos.dedup();
            if VERBOSE {
                println!("{}Valores únicos: {:?}", "    ".repeat(depth+1), valores_unicos); 
            }

            //COMBINACIONES DE VALORES ÚNICOS:
            ramas_combinaciones.push(valores_unicos.clone());
            let mut ramas_combinadas_salida: Vec<String> = Vec::new();
            combinatoria_recursiva(ramas_combinaciones.clone(), &mut ramas_combinadas_salida,
            1, ramas_combinaciones.clone(), ramas.clone());




            let mut new_headers = headers.clone();
            let mut new_vectores = vectores.clone();
            
            
            //Eliminamos la columna actual que coincida con el valor de headers[i_header]:
            new_headers = new_headers.into_iter().filter(|x| *x != headers[i_header]).collect();
            new_vectores = new_vectores.into_iter().filter(|x| x.header != headers[i_header]).collect();
            
            arbol_recursivo(n_headers - 1, new_headers, contador_arboles, depth + 1, 
                n_headers_original, new_vectores.clone(), ramas, ramas_datos, ramas_combinaciones);
        }
    }
}




//FUNCION QUE RECIVE UN VECTOR DE STRINGS Y REALIZA LA COMBINATORIA DE TODOS LOS VALORES DE FORMA RECURSIVA:
fn combinatoria_recursiva(ramas_combinaciones: Vec<Vec<String>>, 
    ramas_combinadas: &mut Vec<String>, 
    depth: usize, 
    ramas_combinaciones_original: Vec<Vec<String>>,
    ramas: Vec<String>,
){
    if ramas_combinaciones.len() != 0 {
        for i in 0..ramas_combinaciones[0].len(){
            ramas_combinadas.push(ramas_combinaciones[0][i].clone());
            if depth == ramas_combinaciones_original.len(){
                println!("{depth}{}Combinación: {:?}", "    ".repeat(depth), ramas_combinadas);
            }
            else{
                combinatoria_recursiva(ramas_combinaciones[1..].to_vec(), ramas_combinadas, depth+1, 
                    ramas_combinaciones_original.clone(), ramas.clone());
            }
            ramas_combinadas.pop(); //ESTO ELIMINA EL ÚLTIMO ELEMENTO DENTRO DE UNA COMBINACIÓN
        }
    }
}








fn read_csv_vector(vector: &mut Vec<Vector>, headers: &mut Vec<String>, path: &str){
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    //Obtenemos los headers:
    let _headers = rdr.headers().unwrap();
    for header in _headers.iter(){
        headers.push(header.to_string());
    }
    println!("{:?}", headers);

    //Implementación correcta:
    for (i, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        for (j, col) in record.iter().enumerate() {
            if i == 0 {
                vector.push(Vector{header: headers[j].to_string(), col: Vec::new() });
            }
            vector[j].col.push(col.to_string());
        }
    }

}
