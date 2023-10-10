use console::{Style, Term};
use dialoguer::{theme::ColorfulTheme, Input};
use std::collections::VecDeque;
use std::fmt::format;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)] //more specific types like [str(name)] todo str to not accept "12"
enum DataType {
    Id,
    Int,
    Str,
    Bool,
    Some(String), //todo autoincrement id
}
#[derive(Debug, Clone)]
struct Header {
    name: String,
    datatype: DataType,
}
impl Header {
    fn new(name: String, datatype: DataType) -> Header {
        match datatype {
            DataType::Some(val) => match val.as_str() {
                "id" => Header {
                    name,
                    datatype: DataType::Id,
                },
                "int" => Header {
                    name,
                    datatype: DataType::Int,
                },
                "str" => Header {
                    name,
                    datatype: DataType::Str,
                },
                "bool" => Header {
                    name,
                    datatype: DataType::Bool,
                },
                _ => panic!("Invalid datatype"),
            },
            _ => Header { name, datatype },
        }
    }
}

struct Column {
    header: Header,
    content: Vec<String>,
}

impl Column {
    fn new(header: Header) -> Column {
        Column {
            header,
            content: vec![],
        }
    }
}

// struct Row{
//     id: i64,
//     content: Vec<datatype> //todo datatype row
// }

struct Table {
    headers: Vec<Header>,
    columns: Vec<Column>,
    // rows: Vec<Row>
}

impl Table {
    fn new(path: &str) -> Table {
        let f = std::fs::read_to_string(path).unwrap();

        let lines_u = f.lines().collect::<Vec<&str>>();
        let mut lines = VecDeque::new();
        for line in lines_u {
            lines.push_back(line.split(",").collect::<Vec<&str>>());
        }

        let headers_u = lines.pop_front().unwrap();
        let mut headers: Vec<Header> = vec![];
        for header_idx in 0..headers_u.len() {
            let split_header = headers_u[header_idx].split('[').collect::<Vec<&str>>();
            let name = split_header[0].to_string();
            let mut datatype = split_header[1].to_string();
            let _ = datatype.pop().unwrap();
            headers.push(Header::new(name, DataType::Some(datatype)));
        }

        let mut table = Table {
            headers,
            columns: vec![],
        };
        table.read_columns_from_lines(&lines);

        table
    }

    fn read_columns_from_lines(&mut self, lines: &VecDeque<Vec<&str>>) {
        for (i, header) in self.headers.iter().enumerate() {
            self.columns.push(Column::new((*header).clone()));
            for line in lines {
                self.columns[i].content.push(line[i].to_string().clone());
            }
        }
    }

    fn add_row(&mut self, row: Vec<String>) {
        for (i, val) in row.iter().enumerate() {
            self.columns[i].content.push(val.to_string().clone());
        }
    }

    fn input_row(&mut self) {
        let mut row = Vec::new();
        for header in self.headers.iter() {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{} with type [{:?}]", header.name, header.datatype))
                .validate_with(|input:&String| {
                    match header.datatype{
                        DataType::Int | DataType::Id => {match input.parse::<i64>() {
                            Ok(val) => Ok(()),
                            Err(e) => Err("should be integer")
                        }},
                        DataType::Str => {match input.parse::<String>() {
                            Ok(val) => Ok(()),
                            Err(e) => Err("should be String")
                        }},
                        DataType::Bool => {match input.parse::<bool>() {
                            Ok(val) => Ok(()),
                            Err(e) => Err("should be bool")
                        }},
                        _ => panic!() // todo more checks for bools to accept "true"
                    }
                })
                .interact_text()
                .unwrap();

            row.push(input)
        }
        self.add_row(row);
    }

    fn select(&mut self, header: &str) -> Result<&Vec<String>, &'static str>{
        for column in self.columns.iter(){
            if column.header.name.as_str() == header {
                return Ok(&column.content);
            }
        }
        Err("empty table or nothing found")
    }
}
fn main() {
    let mut table = Table::new("src/test.rat");
    // table.input_row();
    println!("{:?}", table.select("name").unwrap())
    // todo use dialoguer

}
