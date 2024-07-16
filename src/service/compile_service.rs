use std::collections::HashMap;
use std::fs;
use std::process::Command;

use super::writer_service::write_file;

pub fn compile_cpp_to_assembly(
    data: String,
    functions: Vec<String>,
    compiler: String,
    args: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let file_path = write_file(data);

    let mut output_map: HashMap<String, Vec<String>> = HashMap::new();

    let program_output_name = format!("/storage/program-{}", uuid::Uuid::new_v4());

    // Compile C++ to assembly using g++
    let output = compile_code(
        file_path.clone(),
        program_output_name.to_string(),
        compiler.clone(),
        args.clone()
    );

    match output {
        Ok(_) => {}
        Err(_) => {
            output_map.insert(compiler.clone(), Vec::new());
            return Ok(output_map);
        }
    }

    let disassembly_output = get_assembly(functions.clone(), &program_output_name);

    let output = match disassembly_output {
        Ok(output) => output,
        Err(e) => return Err(e),
    };

    let filtered_output = format_output(output);

    // Clean up the generated assembly file
    fs::remove_file(program_output_name)?;

    output_map.insert(compiler.to_string(), filtered_output);

    fs::remove_file(file_path.clone())?;

    Ok(output_map)
}

fn compile_code(
    file_path: String,
    program_output_name: String,
    compiler: String,
    args: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(compiler)
        .args(&["-g", &file_path, "-o", &program_output_name])
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to compile C++ file: {:?}", output).into());
    }

    Ok(())
}

fn get_assembly(functions: Vec<String>, output_file_name: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    // gdb -batch -ex 'file program' -ex 'disassemble main'
    // ["-batch", "-ex", "disassembly-flavor intel", "-ex", "/storage/program-4d9a2fce-a04a-4aa1-8e63-1a30c564f715", "-ex", "disassemble main"]
    let mut output: Vec<Vec<String>> = Vec::new();

    for function in functions {
        let mut args: Vec<String> = vec![
            "-batch".to_string(),
            "-ex".to_string(),
            "disassembly-flavor intel".to_string(),
            "-ex".to_string(),
            format!("file {}", output_file_name.to_string()),
        ];

        args.push("-ex".to_string());
        args.push("disassemble ".to_owned() + &function);

        println!("{:?}", args);

        let output_local = Command::new("gdb").args(&args).output()?;

        if !output_local.status.success() {
            return Err(format!("Failed to objdump: {:?}", output).into());
        }

        let mut disassembly_lines: Vec<String> = Vec::new();

        disassembly_lines.push(function);

        let temp = String::from_utf8_lossy(&output_local.stdout);

        disassembly_lines.extend(temp.lines().map(String::from));

        disassembly_lines.remove(1);

        output.push(disassembly_lines);
    }

    Ok(output)
}

fn format_output(output: Vec<Vec<String>>) -> Vec<String> {
    let filtered_output: Vec<String> = output
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(|(i, y)| {
                    if i == 0 {
                        x.get(i).unwrap().to_owned()
                    } else {
                        y.split('\t').skip(1).collect::<String>()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect();

    filtered_output
}
