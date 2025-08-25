#[derive(Debug, Clone)]
struct Libro {
    titulo: String,
    autor: String,
    genero: String
}

fn ordenar_libros(libros: &[Libro]) -> Vec<Libro> {
    let mut v = libros.to_vec();
    v.sort_by(|a, b| { 
        let g = a.genero.cmp(&b.genero);
        if g == std::cmp::Ordering::Equal {
            let au = a.autor.cmp(&b.autor);
            if au == std::cmp::Ordering::Equal{
                a.titulo.cmp(&b.titulo)
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
            titulo: "El poder medicinal de las plantas".to_string(),
            autor: "Reinaldo Sosa Gómez".to_string(),
            genero: "Salud".to_string()
        },
        Libro {
            titulo: "Un corazón tranquilo".to_string(),
            autor: "Dr. Carlos Fayard".to_string(),
            genero: "Autoayuda".to_string()
        },
    ];

    let ordenados = ordenar_libros(&libros);

    println!("🔹 Ordenados:");
    for t in &ordenados {
        println!("- {} [{}] ({})", t.titulo, t.autor, t.genero);
    }
}
