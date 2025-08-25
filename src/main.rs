fn sorted_strings(items: &[String]) -> Vec<String> {
    let mut v = items.to_vec();
    v.sort();
    v
}

fn main() {
    let libros = vec![
        String::from("El poder medicinal de las plantas"),
        String::from("Un corazón tranquilo"),
    ];

    let ordenados = sorted_strings(&libros);

    println!("🔹 Original (sin cambios):");
    for t in &libros {
        println!("- {t}");
    }

    println!("🔹 Ordenados:");
    for t in &ordenados {
        println!("- {t}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ordena_copia_sin_mutar_origina() {
        let originales = vec![
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];

        let copia = sorted_strings(&originales);

        assert_eq!(originales, vec!["b", "c", "a"]);
        assert_eq!(copia, vec!["a", "b", "c"]);
    }
}
