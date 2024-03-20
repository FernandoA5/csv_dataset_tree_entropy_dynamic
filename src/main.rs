
#[derive(Debug, Clone)]
struct Vector{
    header: String, //NOMBRE DE LA COLUMNA
    col: Vec<String>, //Vec<ID, VALOR>
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

    arbol_recursivo(n_headers, //CANTIDAD DE COLUMNAS
        headers, //HEADERS
        &mut contador_arboles, //CONTADOR DE ARBOLES
        0, //PROFUNDIDAD INICIAL
        n_headers, //CANTIDAD DE COLUMNAS ORIGINAL
        vectores.clone(), //DATOS
        &mut ramas, //VACIO. AQUÍ SE GUARDARÁN LAS RAMAS CÓMO: [SURVIVED, PCLASS, SEX, ETC] EN ORDEN DEL ÁRBOL
        &mut ramas_datos, //VACIO. INSTANCIAS DE VECTORES.
        &mut ramas_combinaciones //VACIO. AQUÍ SE GUARDARÁN LAS COMBINACIONES DE LAS RAMAS CÓMO: [[0,1,FEMALE],[0,1,MALE],[0,2,FEAMLE], ETC] EN ORDEN DEL ÁRBOL
    );
}


fn arbol_recursivo(n_headers: usize, //CANTIDAD DE COLUMNAS
    headers: Vec<String>, //HEADERS
    contador_arboles: &mut usize, //CONTADOR DE ARBOLES
    depth: usize, //PROFUNDIDAD ACTUAL
    n_headers_original: usize, //CANTIDAD DE COLUMNAS ORIGINAL
    vectores: Vec<Vector>, //DATOS
    ramas: &mut Vec<String>, //RAMAS ACTUALES: [SURVIVED,PCLASS,SEX] || [SURVIVED,SEX,PCLASS] etc
    ramas_datos: &mut Vec<Vector>, //DATOS DE LAS RAMAS ACTUALES
    ramas_combinaciones: &mut Vec<Vec<String>> //COMBINACIONES DE LAS RAMAS ACTUALES
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
            valores_unicos.dedup(); //ELIMINAMOS VALORES REPETIDOS



            if VERBOSE {
                println!("{}Valores únicos: {:?}", "    ".repeat(depth+1), valores_unicos); 
            }

            //COMBINACIONES DE VALORES ÚNICOS:
            ramas_combinaciones.push(valores_unicos.clone());
            let mut ramas_combinadas_salida: Vec<String> = Vec::new();

            combinatoria_recursiva(
                ramas_combinaciones.clone(),  //VALORES ÚNICOS
                ramas_datos,
                &mut ramas_combinadas_salida,
                // &mut datos_ramas_combinadas ,
                1,
                ramas_combinaciones.clone(), 
                ramas.clone(),
                // &mut vec_apariciones
            );


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
fn combinatoria_recursiva(
    ramas_combinaciones: Vec<Vec<String>>, //
    datos_ramas_combinadas_entrada: &mut Vec<Vector>, //DATOS
    ramas_combinadas: &mut Vec<String>, //RAMAS COMBINADAS
    // datos_ramas_combinadas: &mut Vec<String>,
    depth: usize, //PROFUNDIDAD
    ramas_combinaciones_original: Vec<Vec<String>>, //RAMAS COMBINADAS ORIGINAL
    ramas: Vec<String>,
    // vec_apariciones: &mut Vec<Vec<String>>
){
    if ramas_combinaciones.len() != 0  && datos_ramas_combinadas_entrada.len() != 0{
        for i in 0..ramas_combinaciones[0].len(){
            ramas_combinadas.push(ramas_combinaciones[0][i].clone());
            // datos_ramas_combinadas.push(datos_ramas_combinadas_entrada[0].col[i].clone());

            //
            if depth == ramas_combinaciones_original.len(){

                //BUSQUEDA RECURSIVA DE LA COMBINACIÓN:
                let mut apariciones = 0;
                busqueda_recursiva(datos_ramas_combinadas_entrada.clone(), &mut apariciones, ramas_combinadas.clone());
                println!("{depth}{}Apariciones de la Combinación {:?}: {apariciones}", "    ".repeat(depth), ramas_combinadas);
                
            }
            else{
                combinatoria_recursiva(
                    ramas_combinaciones[1..].to_vec(), 
                    datos_ramas_combinadas_entrada,
                    ramas_combinadas, 
                    // datos_ramas_combinadas,
                    depth+1, 
                    ramas_combinaciones_original.clone(), 
                    ramas.clone(),
                    // vec_apariciones
                );
            }
            ramas_combinadas.pop(); //ESTO ELIMINA EL ÚLTIMO ELEMENTO DENTRO DE UNA COMBINACIÓN
        }
    }
}

fn busqueda_recursiva(
    datos_ramas_combinadas_entrada: Vec<Vector>, //DATOS DE LAS RAMAS
    apariciones: &mut usize, //CONTADOR DE APARICIONES
    ramas_combinadas: Vec<String> //LA COMBINACIÓN DE LOS VALORES ÚNICOS DE LA RAMA
){
    //SI EL VECTOR DE DATOS DE LAS RAMAS COMBINADAS NO ESTÁ VACÍO:
    if datos_ramas_combinadas_entrada.len()>1{
        //VECTOR EN EL QUE SE GUARDARÁN LAS COLUMNAS QUE COINCIDAN CON LA COMBINACIÓN DE VALORES ÚNICOS:
        let mut nueva_lista_de_vectores: Vec<Vector> = Vec::new();
        let mut vec_indices_coincidentes: Vec<usize> = Vec::new();
        //RECORREMOS EL PRIMER VECTOR DE DATOS DE LAS RAMAS COMBINADAS:
        for i in 0..datos_ramas_combinadas_entrada[0].col.len(){
            //SI LA COMBINACIÓN DE VALORES ÚNICOS COINCIDE CON EL VALOR DE LA COLUMNA:
            if datos_ramas_combinadas_entrada[0].col[i] == ramas_combinadas[0]{
                //GUARDAMOS EL ÍNDICE DE LA COLUMNA:
                vec_indices_coincidentes.push(i);
            }
        }   
        //RECORREMOS DE NUEVO EL VECTOR DE DATOS DE LAS RAMAS COMBINADAS:
        for i in 0..datos_ramas_combinadas_entrada.len(){
            //RECORREMOS EL VECTOR DE ÍNDICES COINCIDENTES:
            let mut col: Vec<String> = Vec::new();
            for j in 0..vec_indices_coincidentes.len(){
                //GUARDAMOS LOS VALORES DE LAS COLUMNAS QUE COINCIDEN CON LA COMBINACIÓN DE VALORES ÚNICOS:
                col.push(datos_ramas_combinadas_entrada[i].col[vec_indices_coincidentes[j]].clone());
            }
            //GUARDAMOS LOS VALORES DE LAS COLUMNAS QUE COINCIDEN CON LA COMBINACIÓN DE VALORES ÚNICOS:
            nueva_lista_de_vectores.push(Vector{header: datos_ramas_combinadas_entrada[i].header.clone(), col: col});
        }

        //UNA VEZ QUE TENEMOS EL NUEVO VECTOR, LO PASAMOS A LA FUNCIÓN DE BUSQUEDA RECURSIVA:
        // println!("Nueva Lista de vectores: {:?}", nueva_lista_de_vectores);
        busqueda_recursiva(
            nueva_lista_de_vectores[1..].to_vec(), //ELIMINAMOS EL PRIMER ELEMENTO DE LA COMBINACIÓN
            apariciones,  //PASAMOS EL CONTADOR DE APARICIONES
            ramas_combinadas[1..].to_vec() //ELIMINAMOS EL PRIMER ELEMENTO DE LA COMBINACIÓN
        );
        
    }
    else{
        //SI SOLO QUE DA UN VECTOR DE DATOS DE LAS RAMAS COMBINADAS:
        for i in 0..datos_ramas_combinadas_entrada[0].col.len(){
            // println!("{:?}", datos_ramas_combinadas_entrada[0].col[i] );
            // println!("{:?}", ramas_combinadas[0] );
            if datos_ramas_combinadas_entrada[0].col[i] == ramas_combinadas[0]{
                *apariciones += 1;
            }
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
    let mut id: usize = 0;
    //Implementación correcta:
    for (i, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        for (j, col) in record.iter().enumerate() {
            if i == 0 {
                vector.push(Vector{header: headers[j].to_string(), col: Vec::new() });
            }
            vector[j].col.push(col.to_string());
            id += 1;
        }
    }

}
