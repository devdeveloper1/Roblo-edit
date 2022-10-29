use std::ffi::OsString;
use std::fs;
use std::io;
use std::path;

extern crate dirs;

fn main() {
    let roblox: String;

    let images_exists = path::Path::new("./imagens").exists();
    struct Face {
        name: OsString,
        index: u32,
        path: path::PathBuf,
    }

    let mut temp_face = path::Path::new("");

    if !images_exists {
        fs::create_dir("./imagens").expect("não foi possivel criar o diretorio imagens");
    }

    temp_face = path::Path::new("./temp/face.png");

    match dirs::data_local_dir() {
        Some(path) => {
            roblox = format!("{}{}", path.display(), "\\Roblox\\versions");
            println!("local das versões do Roblox:{}", roblox);
            println!(
                "\nbem vindo ao roblo-edit, um programa aonde voçê pode mudar sua face do Roblox!\n"
            );
            println!("importante: você ira ter que aperta o numero da face que você quer\npor causa de um bug que eu não consegui consertar :>");
            println!("\npara ter mais opções de faces e so colocar mais imagens na pasta imagens (precisa ser .png)");

            for version in fs::read_dir(roblox).expect("houve um erro ao ler as versões") {
                if let Ok(version) = version {
                    if path::Path::new(&format!(
                        "{}\\content\\textures\\face.png",
                        version.path().display()
                    ))
                    .exists()
                    {
                        let mut i: u32 = 0;
                        let mut faces: Vec<Face> = Vec::new();

                        for face in
                            fs::read_dir(".\\imagens").expect("houve um erro ao ler as imagens")
                        {
                            i += 1;

                            if let Ok(face) = face {
                                let face2 = Face {
                                    index: i,
                                    name: face.file_name(),
                                    path: face.path(),
                                };

                                faces.push(face2);
                            }
                        }

                        if true {
                            println!("faces disponiveis:\n");

                            for fac in &faces {
                                let face_name = fac
                                    .name
                                    .to_str()
                                    .expect("não foi possivel converter o nome da face");

                                println!("{}: {}", fac.index, face_name);
                            }
                            let mut face_escolhida = String::new();

                            io::stdin()
                                .read_line(&mut face_escolhida)
                                .expect("o programa não conseguiu ler a linha");

                            let face_escolhida: u32 = face_escolhida
                                .trim()
                                .parse()
                                .expect("por favor insira um numero!");

                            for fac in &faces {
                                println!("{}", fac.path.display());

                                println!("{}", face_escolhida);

                                if face_escolhida == fac.index {
                                    let face_path: &str = &format!(
                                        "{}\\content\\textures\\face.png",
                                        version.path().display()
                                    );

                                    fs::copy(fac.path.as_path(), face_path)
                                        .expect("não foi possivel copiar do temp para o face.png");
                                }
                            }
                        }
                    }
                }
            }
        }

        None => println!("aconteceu um erro, por favor verifique que você tem o Roblox instalado"),
    }
}
