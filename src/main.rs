use wasmtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let filename = args
        .get(0)
        .map_or_else(|| Err("missing argument: filename"), Ok)?;

    let module_bytes = std::fs::read(filename)?;

    let engine = Engine::default();

    let module = Module::new(&engine, module_bytes)?;

    println!("exports:");
    for export in module.exports() {
        print_extern_type(export.name(), export.ty())
    }

    println!();
    println!("imports:");
    for import in module.imports() {
        print_extern_type(format!("{} {}", import.module(), import.name()).as_str(), import.ty())
    }

    Ok(())
}

fn print_extern_type(name: &str, et: ExternType) {
        match et {
            ExternType::Func(f) => {
                print!("func {}({})", name, f.params().map(|f|f.to_string()).collect::<Vec<String>>().join(", "));
                match f.results().collect::<Vec<ValType>>().as_slice() {
                    [] => println!(),
                    [res] => println!(" -> {}", res),
                    a => {
                        println!(" -> ({})", a.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", "))
                    }
                }
            },
            ExternType::Global(g) => {
                print!("gloabl {} {}", name, g.content());
                match g.mutability() {
                    Mutability::Const => println!(" (mutable)"),
                    Mutability::Var => println!(" (constant)"),
                }
            },
            ExternType::Memory(m) => println!("memory '{}' {:?}", name, m),
            ExternType::Table(t) => println!("table {} {:?}", name, t),
        }
}