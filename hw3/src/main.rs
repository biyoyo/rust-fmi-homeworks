fn main() {
    println!("{:?}", skip_next("foo", 'f'));
    println!("{:?}", skip_next("foo", 'a'));
    println!("{:?}", take_and_skip(" го/шо ", '/'));
    println!("{:?}", take_and_skip(" го/шо ", 'о'));
    println!("{:?}", take_and_skip(" го/шо ", 'a'));
}
pub fn skip_next(input: &str, target: char) -> Option<&str> {
    let mut chars = input.chars();
    if chars.next().unwrap() == target {
        return Some(chars.as_str());
    }
    None
}

pub fn take_until(input: &str, target: char) -> (&str, &str) {
    if input.find(target) == None {
        return (input, "");
    }
    let mut char_indices = input.char_indices();
    let char_len: usize;
    loop {
        let value = char_indices.next().unwrap();
        if value.1 == target {
            char_len = value.1.len_utf8();
            break;
        }
    }
    let pos = match char_indices.next() {
        Some(ch) => ch.0,
        None => input.len(),
    };
    return input.split_at(pos - char_len);
}

pub fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
    input.find(target)?;
    let (first, second) = take_until(input, target);
    Option::Some((
        first,
        skip_next(second, second.chars().next().unwrap()).unwrap(),
    ))
}

#[derive(Debug)]
pub enum CsvError {
    IO(std::io::Error),
    ParseError(String),
    InvalidHeader(String),
    InvalidRow(String),
    InvalidColumn(String),
}

use std::collections::HashMap;

type Row = HashMap<String, String>;

use std::io::BufRead;

pub struct Csv<R: BufRead> {
    pub columns: Vec<String>,
    reader: R,
    selection: Option<Box<dyn Fn(&Row) -> Result<bool, CsvError>>>,
}

use std::io::Write;

impl<R: BufRead> Csv<R> {
    pub fn new(mut reader: R) -> Result<Self, CsvError> {
        let mut buf = String::new();

        match reader.read_line(&mut buf) {
            Err(e) => return Err(CsvError::IO(e)),
            Ok(0) => return Err(CsvError::InvalidHeader(String::from("No header"))),
            _ => (),
        }

        let columns: Vec<String> = buf
            .split(',')
            .map(|column| column.trim().to_string())
            .collect();

        for i in 0..columns.len() {
            for j in i + 1..columns.len() {
                if columns[i] == columns[j] {
                    return Err(CsvError::InvalidHeader(String::from(
                        "Duplicate column names",
                    )));
                }
            }
        }
        Ok(Csv {
            columns,
            reader,
            selection: None,
        })
    }

    pub fn parse_line(&mut self, line: &str) -> Result<Row, CsvError> {
        let line = line.trim();

        let mut row = Row::new();

        let mut line = match skip_next(line, '"') {
            Some(s) => s,
            None => {
                return Err(CsvError::InvalidRow(String::from(
                    "Invalid beginning of row",
                )))
            }
        };

        for (i, col) in self.columns.iter().enumerate() {
            let (value, remainder) = match take_and_skip(line, '"') {
                Some(s) => s,
                None => {
                    return Err(CsvError::InvalidRow(String::from(
                        "Missing closing quotation mark",
                    )))
                }
            };
            line = remainder;

            row.insert(col.clone(), value.to_string());

            if i == self.columns.len() - 1 {
                if !line.is_empty() {
                    return Err(CsvError::InvalidRow(String::from(
                        "Not enough values in row",
                    )));
                }
            } else {
                let (delim, remainder) = match take_and_skip(line, '"') {
                    Some(s) => s,
                    None => {
                        return Err(CsvError::InvalidRow(String::from(
                            "Missing opening quotation mark",
                        )))
                    }
                };
                line = remainder;

                if delim.trim() != "," {
                    return Err(CsvError::InvalidRow(String::from("Delimiter error")));
                }
            }
        }
        Ok(row)
    }

    pub fn apply_selection<F>(&mut self, callback: F)
    where
        F: Fn(&Row) -> Result<bool, CsvError> + 'static,
    {
        self.selection = Some(Box::new(callback));
    }

    pub fn write_to<W: Write>(mut self, mut writer: W) -> Result<(), CsvError> {
        let cols = self.columns.join(", ") + "\n";
        if let Err(e) = writer.write(cols.as_bytes()) {
            return Err(CsvError::IO(e));
        }

        while let Some(row) = self.next() {
            let row = row?;

            let mut line = String::new();

            for (i, col) in self.columns.iter().enumerate() {
                line += "\"";
                line += row.get(col).unwrap();
                line += "\"";

                if i != self.columns.len() - 1 {
                    line += ", ";
                }
            }
            line += "\n";

            if let Err(e) = writer.write(line.as_bytes()) {
                return Err(CsvError::IO(e));
            }
        }
        Ok(())
    }
}

impl<R: BufRead> Iterator for Csv<R> {
    type Item = Result<Row, CsvError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Err(e) => return Some(Err(CsvError::IO(e))),
                Ok(0) => return None,
                _ => (),
            };

            let row = match self.parse_line(&line) {
                Ok(row) => row,
                Err(e) => return Some(Err(e)),
            };

            if self.selection.is_none() {
                return Some(Ok(row));
            }

            match self.selection.as_ref().unwrap()(&row) {
                Ok(true) => return Some(Ok(row)),
                Ok(false) => (),
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::skip_next;
    use crate::take_and_skip;
    use crate::take_until;
    use crate::Csv;
    use crate::CsvError;

    use std::io::Cursor;
    use std::io::{self, BufRead, BufReader, Read};

    #[test]
    fn parse_text_test() {
        assert_eq!(skip_next("foo", 'f'), Some("oo"));
        assert_eq!(skip_next("foo", 'a'), None);
        assert_eq!(take_until("one/", '/'), ("one", "/"));
        assert_eq!(take_until("one/two", '/'), ("one", "/two"));
        assert_eq!(take_until("onetwo", '/'), ("onetwo", ""));
        assert_eq!(take_and_skip(" го/шо ", '/'), Some((" го", "шо ")));
        assert_eq!(take_and_skip(" го/шо ", 'о'), Some((" г", "/шо ")));
        assert_eq!(take_and_skip(" го/шa", 'a'), Some((" го/ш", "")));
        assert_eq!(take_and_skip(" го/шо ", 'a'), None);
        assert_eq!(
            take_and_skip(",'12','13','14'", '\''),
            Some((",", "12','13','14'"))
        );
        assert_eq!(take_and_skip("asdf,", ','), Some(("asdf", "")));
    }

    fn csv_from_str(s: &str) -> Result<Csv<BufReader<Cursor<String>>>, CsvError> {
        Csv::new(BufReader::new(Cursor::new(String::from(s))))
    }

    #[test]
    fn test_new() {
        let c = csv_from_str("name, age, date").unwrap();
        assert_eq!(c.columns, vec!["name", "age", "date"]);

        assert!(matches!(csv_from_str(""), Err(CsvError::InvalidHeader(_))));
        assert!(matches!(
            csv_from_str("name, name, age, some, name"),
            Err(CsvError::InvalidHeader(_))
        ));

        assert_eq!(
            csv_from_str("name,age,date").unwrap().columns,
            vec!["name", "age", "date"]
        );
    }

    #[test]
    fn test_parse_row() {
        let mut c = csv_from_str("name, age, date").unwrap();
        let row = c.parse_line("\"gosho\",\"17\",\"17.10\"").unwrap();
        assert_eq!(row.get("name"), Some(&String::from("gosho")));
        assert_eq!(row.get("age"), Some(&String::from("17")));
        assert_eq!(row.get("date"), Some(&String::from("17.10")));
        let row = c
            .parse_line("\"gosho, hello\" , \"17\", \"17.10\"")
            .unwrap();
        assert_eq!(row.get("name"), Some(&String::from("gosho, hello")));
        assert_eq!(row.get("age"), Some(&String::from("17")));
        assert_eq!(row.get("date"), Some(&String::from("17.10")));
    }

    #[test]
    fn next_test() {
        let reader = BufReader::new(
            r#"
        name, age, birth date
        "Douglas Adams", "42", "1952-03-11"
        "Gen Z. Person", "20", "2000-01-01"
        "Ada Lovelace", "36", "1815-12-10"
        "#
            .trim()
            .as_bytes(),
        );

        // Конструираме си CSV-то:
        let mut csv = Csv::new(reader).unwrap();
        csv.apply_selection(|row| {
            let age = row
                .get("age")
                .ok_or_else(|| CsvError::InvalidColumn(String::from("age")))?;
            let age = age
                .parse::<u32>()
                .map_err(|_| CsvError::ParseError(String::from(age)))?;

            Ok(age > 30)
        });

        assert_eq!(
            csv.next().unwrap().unwrap().get("name").unwrap(),
            "Douglas Adams"
        );
        assert_eq!(
            csv.next().unwrap().unwrap().get("name").unwrap(),
            "Ada Lovelace"
        );
        assert!(csv.next().is_none());
    }

    // Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
    // на този ред:

    // За тестване че някакъв резултат пасва на някакъв pattern:
    macro_rules! assert_match {
        ($expr:expr, $pat:pat) => {
            if let $pat = $expr {
                // all good
            } else {
                assert!(
                    false,
                    "Expression {:?} does not match the pattern {:?}",
                    $expr,
                    stringify!($pat)
                );
            }
        };
    }

    // За тестване на IO грешки:
    struct ErroringReader {}

    impl Read for ErroringReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "read error!"))
        }
    }

    impl BufRead for ErroringReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
        }

        fn consume(&mut self, _amt: usize) {}
    }

    #[test]
    fn test_string_parsing() {
        assert_eq!(skip_next("[test]", '['), Some("test]"));
        assert_eq!(take_until("one/two", '/'), ("one", "/two"));
        assert_eq!(take_and_skip("one/two", '/'), Some(("one", "two")));
    }

    #[test]
    fn test_csv_error() {
        assert_match!(Csv::new(ErroringReader {}).err(), Some(CsvError::IO(_)));
    }

    #[test]
    fn test_basic_csv() {
        let data = r#"
            name, age, birth date
            "Gen Z. Person", "20", "2000-01-01"
            "Douglas Adams", "42", "1952-03-11"
            "Ada Lovelace", "36", "1815-12-10"
        "#
        .trim()
        .as_bytes();

        let mut csv = Csv::new(BufReader::new(data)).unwrap();
        csv.apply_selection(|_row| Ok(true));

        // Парсене на един ред:
        let row = csv.parse_line(r#""Basic Name","13","2020-01-01""#).unwrap();
        assert_eq! {
            (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
            ("Basic Name", "13", "2020-01-01"),
        };

        // Употреба като итератор:
        let filtered_names = csv
            .map(|row| row.unwrap()["name"].clone())
            .collect::<Vec<_>>();
        assert_eq!(
            filtered_names,
            &["Gen Z. Person", "Douglas Adams", "Ada Lovelace"]
        );

        // Писане в някакъв изход
        let mut csv = Csv::new(BufReader::new(data)).unwrap();
        csv.apply_selection(|_row| Ok(true));

        let mut output = Vec::new();
        csv.write_to(&mut output).unwrap();

        let output_lines = output.lines().map(Result::unwrap).collect::<Vec<String>>();

        assert_eq!(
            output_lines,
            &[
                "name, age, birth date",
                "\"Gen Z. Person\", \"20\", \"2000-01-01\"",
                "\"Douglas Adams\", \"42\", \"1952-03-11\"",
                "\"Ada Lovelace\", \"36\", \"1815-12-10\"",
            ]
        );
    }
}
