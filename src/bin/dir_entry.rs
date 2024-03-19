use std::io::Result;


fn main() -> Result<()> {
    let dir_path: std::path::PathBuf = std::env::args()
        .nth(1).unwrap()
        .into();
    let dir_path = dir_path.canonicalize()?;

    fn fetch_entries(
        dir: std::path::PathBuf
    ) -> std::io::Result<Vec<std::path::PathBuf>> {
        dir.read_dir()?
            .map(|de| de.map(|de| de.path()))
            .collect()
    }

    let mut files = Vec::new(); {
        let mut entries = fetch_entries(dir_path.clone())?;
        while let Some(entry) = entries.pop() {
            if entry.is_file() {
                let path_sections = entry.canonicalize()?
                    .components()
                    .skip(dir_path.components().count())
                    .map(|c| c.as_os_str().to_os_string()
                        .into_string()
                        .map_err(|os_string| std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Can't read a path segment `{}`", os_string.as_encoded_bytes().escape_ascii())
                        ))
                    )
                    .collect::<std::io::Result<Vec<_>>>()?;

                if path_sections.last().unwrap().starts_with('.') {
                    eprintln!("\
                        =========\n\
                        [WARNING] `Route::Dir`: found `{}` in directory `{}`, \
                        are you sure to serve this file？\n\
                        =========\n",
                        entry.display(),
                        dir_path.display(),
                    )
                }
 
                files.push((
                    path_sections,
                    std::fs::File::open(entry)?
                ));

            } else if entry.is_dir() {
                entries.append(&mut fetch_entries(entry)?)

            } else {
                continue
            }
        }
    }

    println!("{files:#?}");

    Ok(())
}
