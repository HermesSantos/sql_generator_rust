use std::fs::{read_dir, DirEntry};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Caminho para o repositório onde estão as imagens
    let repo_path = "/home/hermes/Desktop/projects/admin-exercita/well-known/assets/exercises_images/Exercise_images";
    let output_file_path = "sql_output.txt";

    // Abre ou cria o arquivo de saída
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file_path)?;

    // Lê o diretório especificado
    if let Ok(entries) = read_dir(Path::new(repo_path)) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Escreve a instrução SQL no arquivo de saída
                write_insert_statement(&mut file, entry)?;
            }
        }
    }

    println!(
        "Script concluído com sucesso! Os resultados estão no arquivo {}",
        output_file_path
    );

    Ok(())
}

fn write_insert_statement(file: &mut std::fs::File, entry: DirEntry) -> std::io::Result<()> {
    let file_name = entry.file_name().to_string_lossy().to_string();
    writeln!(
        file,
        "INSERT INTO new_exercises_images (exercise_title, exercise_image, exercise_type) VALUES ('{}', '{}', 'stretching');",
        file_name, file_name
    )
}
