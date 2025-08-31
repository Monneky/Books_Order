#[derive(Debug, Clone)]
struct Libro {
    titulo: String,
    autor: String,
    genero: String,
    saga_order: Option<i32>,
    calification: Option<f32>
}

fn ordenar_libros(libros: &[Libro]) -> Vec<Libro> {
    let mut v = libros.to_vec();
    v.sort_by(|a, b| { 
        let g = a.genero.cmp(&b.genero);
        if g == std::cmp::Ordering::Equal {
            let au = a.autor.cmp(&b.autor);
            if au == std::cmp::Ordering::Equal{
                let so = a.saga_order.cmp(&b.saga_order);
                if so == std::cmp::Ordering::Equal{
                    a.titulo.cmp(&b.titulo)
                }else{
                    so
                }
            }else{
                au
            }
        }else {
            g
        }
    });
    v
}

fn main() {
    let libros = vec![
        Libro {
            titulo: "Ángel mecánico".to_string(),
            autor: "Cassandra Clare".to_string(),
            genero: "Fantasía".to_string(),
            saga_order: Some(2),
            calification: Some(4.5)
        },
        Libro {
            titulo: "Poeta en Nueva York".to_string(),
            autor: "Federico García Lorca".to_string(),
            genero: "Poética".to_string(),
            saga_order: None,
            calification: Some(2.0)
        },
        Libro {
            titulo: "Tiempo de alacranes".to_string(),
            autor: "Bernardo Fernández".to_string(),
            genero: "Novela Policiaca".to_string(),
            saga_order: Some(1),
            calification: Some(5.0)
        },
        Libro {
            titulo: "Hielo negro".to_string(),
            autor: "Bernardo Fernández".to_string(),
            genero: "Novela Policiaca".to_string(),
            saga_order: Some(2),
            calification: Some(4.7)
        },
        Libro {
            titulo: "Cuello blanco".to_string(),
            autor: "Bernardo Fernández".to_string(),
            genero: "Novela Policiaca".to_string(),
            saga_order: Some(3),
            calification: Some(4.5)
        },
        Libro {
            titulo: "Azul Cobalto".to_string(),
            autor: "Bernardo Fernández".to_string(),
            genero: "Novela Policiaca".to_string(),
            saga_order: Some(4),
            calification: Some(4.5)
        },
        Libro {
            titulo: "Effortless English: Learn To Speak English Like A Native".to_string(),
            autor: "A.J. Hoge".to_string(),
            genero: "Lenguajes".to_string(),
            saga_order: None,
            calification: Some(1.0)
        },
        Libro {
            titulo: "El padrino".to_string(),
            autor: "Mario Puzo".to_string(),
            genero: "Novela".to_string(),
            saga_order: None,
            calification: Some(4.0)
        },
        Libro {
            titulo: "Piezas de anime para piano".to_string(),
            autor: "Varios".to_string(),
            genero: "Música".to_string(),
            saga_order: Some(1),
            calification: None
        },
    ];

    let ordenados = ordenar_libros(&libros);

    println!("🔹 Ordenados:");
    for t in &ordenados {
        println!("- {} [{}] ({}) - Calificación: {}", t.titulo, t.autor, t.genero, t.calification.map_or("No calificado".to_string(), |c| format!("{:.1}", c)));
    }
}
