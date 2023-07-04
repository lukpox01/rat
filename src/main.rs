use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum DataType {
    Id,
    Int,
    Str,
    Bool,
    Some(String),//todo autoincrement id
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
                self.columns[i].content.push(line[i].to_string());
            }
        }
    }
}

fn main() {
    let table = Table::new("src/test.rat");


}
