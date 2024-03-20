


#[derive(Debug, Clone)]
struct Vector{
    header: String, //NOMBRE DE LA COLUMNA
    col: Vec<String>, //Vec<ID, VALOR>
}
#[derive(Debug, Clone)]
struct Resultado{
    combinacion: String,
    apariciones: usize,
    probabilidad: f64,
    entropia: f64,
}
#[derive(Debug, Clone)]
struct ResultadoArbol{
    arbol: String,
    resultados: Vec<Resultado>,
}

const PATH: &str = "src/examen.csv";
const VERBOSE: bool = false;

fn main() {
    let mut headers: Vec<String> = Vec::new();
    let mut vectores: Vec<Vector> = Vec::new();
    read_csv_vector(&mut vectores, &mut headers, PATH);
    
    //Obtenemos la combinatoria de las columnas:
    let combinaciones: usize= (1..=headers.len()).product();
    println!("Arboles: {}", combinaciones); //To-delete
    

    //VERSIÓN DINÁMICA:
    println!("\n########################----VERSIÓN DINÁMICA----########################");

    let mut resultado_por_arbol: Vec<ResultadoArbol> = Vec::new();

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
        &mut ramas_combinaciones, //VACIO. AQUÍ SE GUARDARÁN LAS COMBINACIONES DE LAS RAMAS CÓMO: [[0,1,FEMALE],[0,1,MALE],[0,2,FEAMLE], ETC] EN ORDEN DEL ÁRBOL
        &mut resultado_por_arbol
    );

    // //IMPRIMIMOS LOS RESULTADOS:
    for resultado_arbol in resultado_por_arbol.clone(){
        println!("Arbol: {}", resultado_arbol.arbol);
        for resultado in resultado_arbol.resultados.clone(){
            println!("Combinación: {}, Apariciones: {}, Probabilidad: {}, Entropía: {}", resultado.combinacion, resultado.apariciones, resultado.probabilidad, resultado.entropia);
        }
        //ENTROPÍA TOTAL:
        let mut entropia_total: f64 = 0.0;
        for resultado in resultado_arbol.resultados.clone(){
            entropia_total += resultado.entropia;
        }
        println!("Entropía Total: {}", entropia_total);

        println!("-------------------------------------------------");
    }

    //PREGUNTAMOS AL USUARIO SI QUIERE VER EL ARBOL CON MENOR ENTROPÍA:
    println!("¿Desea ver el árbol con menor entropía? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();


    if input == "y" || input == "Y" {

        //ARBOL DE MENOR ENTROPÍA:
        let mut menor_entropia: f64 = 100.0;
        let mut arbol_menor_entropia: ResultadoArbol = ResultadoArbol{
            arbol: "".to_string(),
            resultados: Vec::new()
        };
        for resultado_arbol in resultado_por_arbol{
            let mut entropia_total: f64 = 0.0;

            //SEPARAMOS SU NOMBRE DE ARBOL PARA OBTENER LA CANTIDAD DE RAMAS PARTIENDO DE UN split sobre "»"
            let mut arbol: Vec<&str> = resultado_arbol.arbol.split(" » ").collect();
            arbol.pop(); //ELIMINAMOS EL ÚLTIMO ELEMENTO QUE SIEMPRE SERÁ UN STRING VACÍO
            let cantidad_ramas = arbol.len();


            if  cantidad_ramas == n_headers - 1{
                for resultado in resultado_arbol.resultados.clone(){
                    entropia_total += resultado.entropia;
                }
                if entropia_total < menor_entropia && entropia_total != 0.0{
                    menor_entropia = entropia_total;
                    arbol_menor_entropia = resultado_arbol;
                } 
            }
        }
        println!("Arbol con menor entropía: {}", arbol_menor_entropia.arbol);
        let entropy: f64 = arbol_menor_entropia.resultados.iter().map(|x| x.entropia).sum();
        for resultado in arbol_menor_entropia.resultados.clone(){
            println!("Combinación: {}, Apariciones: {}, Probabilidad: {}, Entropía: {}", resultado.combinacion, resultado.apariciones, resultado.probabilidad, resultado.entropia);
        }
        println!("Entropía Total: {}", entropy);
        println!("-------------------------------------------------");
    }

}


fn arbol_recursivo(n_headers: usize, //CANTIDAD DE COLUMNAS
    headers: Vec<String>, //HEADERS
    contador_arboles: &mut usize, //CONTADOR DE ARBOLES
    depth: usize, //PROFUNDIDAD ACTUAL
    n_headers_original: usize, //CANTIDAD DE COLUMNAS ORIGINAL
    vectores: Vec<Vector>, //DATOS
    ramas: &mut Vec<String>, //RAMAS ACTUALES: [SURVIVED,PCLASS,SEX] || [SURVIVED,SEX,PCLASS] etc
    ramas_datos: &mut Vec<Vector>, //DATOS DE LAS RAMAS ACTUALES
    ramas_combinaciones: &mut Vec<Vec<String>>, //COMBINACIONES DE LAS RAMAS ACTUALES
    resultados_por_arbol: &mut Vec<ResultadoArbol> //AQUÍ SE GUARDARÁN LOS RESULTADOS
) {
    if n_headers != 0 {
        for i_header in 0..headers.len(){
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

            //-------------------------------------CALCULOS-------------------------------------//
            
            //COMBINACIONES DE VALORES ÚNICOS:
            let mut resultados: Vec<Resultado> = Vec::new();
            
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
                &mut resultados,
                resultados_por_arbol
            );

            //GUARDAMOS EL RESULTADO DEL ARBOL:
            let resultado_arbol: ResultadoArbol = ResultadoArbol{
                arbol: ramas.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" » "),
                resultados: resultados.clone()
            };
            resultados_por_arbol.push(resultado_arbol);




            let mut new_headers = headers.clone();
            let mut new_vectores = vectores.clone();
            
            
            //Eliminamos la columna actual que coincida con el valor de headers[i_header]:
            new_headers = new_headers.into_iter().filter(|x| *x != headers[i_header]).collect();
            new_vectores = new_vectores.into_iter().filter(|x| x.header != headers[i_header]).collect();
            
            arbol_recursivo(
                n_headers - 1, 
                new_headers, 
                contador_arboles, 
                depth + 1, 
                n_headers_original, 
                new_vectores.clone(), 
                ramas, 
                ramas_datos, 
                ramas_combinaciones,
                resultados_por_arbol
            );
        }
    }
}




//FUNCION QUE RECIVE UN VECTOR DE STRINGS Y REALIZA LA COMBINATORIA DE TODOS LOS VALORES DE FORMA RECURSIVA:
fn combinatoria_recursiva(
    ramas_combinaciones: Vec<Vec<String>>, //CONTIENE LOS VALORES ÚNICOS DE LAS RAMAS (["S", "C", "Q"])
    datos_ramas_combinadas_entrada: &mut Vec<Vector>, //DATOS
    ramas_combinadas: &mut Vec<String>, //CONTIENE LOS VALORES DE LAS RAMAS COMBINADAS (["S", "male", "3", "0"])
    // datos_ramas_combinadas: &mut Vec<String>,
    depth: usize, //PROFUNDIDAD
    ramas_combinaciones_original: Vec<Vec<String>>, //RAMAS COMBINADAS ORIGINAL
    ramas: Vec<String>, //CONTIENE EL ORDEN DE LAS RAMAS DEL ARBOL (["SURVIVED"]) || (["SURVIVED", "PCLASS", "SEX", EMBARKED"])
    resultado_apariciones: &mut Vec<Resultado>, //AQUÍ SE GUARDARÁN LOS RESULTADOS DE LAS APARICIONES
    resultados_por_arbol: &mut Vec<ResultadoArbol> //AQUÍ SE GUARDARÁN LOS RESULTADOS
){
    if ramas_combinaciones.len() != 0  && datos_ramas_combinadas_entrada.len() != 0{
        for i in 0..ramas_combinaciones[0].len(){
            ramas_combinadas.push(ramas_combinaciones[0][i].clone());
            // datos_ramas_combinadas.push(datos_ramas_combinadas_entrada[0].col[i].clone());

            //SI LLEGAMOS A LA PROFUNDIDAD DE LA CANTIDAD DE RAMAS COMBINADAS:
            if depth == ramas_combinaciones_original.len(){

                //RESULTADO VACÍO:
                let mut resultado: Resultado = Resultado{
                    combinacion: "".to_string(), 
                    apariciones: 0, 
                    probabilidad: 0.0, 
                    entropia: 0.0
                };

                //BUSQUEDA RECURSIVA DE LA COMBINACIÓN:
                let mut apariciones = 0;
                busqueda_recursiva(datos_ramas_combinadas_entrada.clone(), &mut apariciones, ramas_combinadas.clone());
                
                if VERBOSE{
                    println!("{depth}{}Apariciones de la Combinación {:?}: {apariciones}", "    ".repeat(depth), ramas_combinadas);
                }

                //GUARDAMOS LA CANITDAD DE APARICIONES DE LA COMBINACIÓN EN EL RESULTADO:
                let str_combinación = ramas_combinadas.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" » ");
                
                resultado.combinacion = str_combinación.clone();
                resultado.apariciones = apariciones;
                
                
                //AHORA, PARA ESA CANTIAD DE APARICIONES, CALCULAMOS LA PROBABILIDAD:
                //HAY QUE TENER CUIDADO CON LA CANTIDAD TOTAL DE APARICIONES:
                //DADO QUE DEPENDE DE LA CANTIDAD DE APARICIONES DE LA COMBINACIÓN DE LA RAMA ANTERIOR:
                let mut total_apariciones: usize = 0;
                //BUSCAMOS LA CANTIDAD DE APARICIONES DE LA COMBINACIÓN DE LA RAMA ANTERIOR:
                //SI EL VECTOR DE RESULTADOS NO ESTÁ VACÍO:
            
                //BUSCAMOS LA CANTIDAD DE APARICIONES DE LA COMBINACIÓN DE LA RAMA ANTERIOR:
                let mut encontrado = false;
                let str_ramas = ramas[0..ramas.len()-1].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" » ");
                
                for i in 0..resultados_por_arbol.len(){
                    
                    //BUSCAMOS UN ARBOL QUE TENGA LAS MISMAS RAMAS QUE LA COMBINACIÓN DE LA RAMA ANTERIOR:
                    if resultados_por_arbol[i].arbol == str_ramas{
                        //BUSCAMOS UNA COMBINACIÓN QUE TENGA LAS MISMAS RAMAS QUE LA COMBINACIÓN DE LA RAMA ANTERIOR:
                        //SI EL ANTERIOR ERA SURVIVED » PCLASS, DEBEMOS ESTAR SEGUROS DE QUE
                        // TENGAMOS EL SURVIVED » PCLASS » 1, SURVIVED » PCLASS » 2, ETC
                        for resultado_arbol in &resultados_por_arbol[i].resultados{
                            let str_combinación = ramas_combinadas[0..ramas_combinadas.len()-1].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" » ");
                            // println!("{} vs {str_combinación}", resultado_arbol.combinacion );
                            if resultado_arbol.combinacion == str_combinación{
                                total_apariciones = resultado_arbol.apariciones;
                                encontrado = true;
                            }
                        }
                    }
                    // return; //to-delete
                    
                }
                if !encontrado{
                    //USAMOS LA TOTALIDAD DE LOS DATOS DEL CSV
                    total_apariciones = datos_ramas_combinadas_entrada[0].col.len();
                }

                // println!("{depth}{}Total de Apariciones de la Combinación Anterior: {total_apariciones}", "    ".repeat(depth)); //to-delete

                //GUARDAMOS LA PROBABILIDAD EN EL RESULTADO:
                //VERIFICAMOS QUE NO SEA 0 PARA EVITAR DIVISIÓN ENTRE 0:
                if total_apariciones != 0{
                    resultado.probabilidad = resultado.apariciones as f64 / total_apariciones as f64;
                }
                else{
                    resultado.probabilidad = 0.0;
                }

                //IMPRIMIMOS LA PROBABILIDAD:
                if VERBOSE{
                    println!("{depth}{}Probabilidad de la Combinación {:?}: {probabilidad}", "    ".repeat(depth), ramas_combinadas, probabilidad=resultado.probabilidad);
                }

                //CALCULAMOS LA ENTROPIA:
                if resultado.probabilidad != 0.0{
                    resultado.entropia = -1.0 * resultado.probabilidad * resultado.probabilidad.log10();
                    if resultado.entropia == -0.0{
                        resultado.entropia = 0.0;
                    }
                }
                else{
                    resultado.entropia = 0.0;
                }

                //IMPRIMIMOS LA ENTROPIA:
                if VERBOSE{
                    println!("{depth}{}Entropía de la Combinación {:?}: {entropia}", "    ".repeat(depth), ramas_combinadas, entropia=resultado.entropia);
                }

            


                
                //UNA VEZ QUE CALCULAMOS TODO, REPORAMOS EL RESULTADO:
                resultado_apariciones.push(resultado);
                // vec_apariciones.push(datos_ramas_combinadas.clone());
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
                    resultado_apariciones,
                    resultados_por_arbol
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
