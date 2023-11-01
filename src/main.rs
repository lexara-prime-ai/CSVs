use csv::{Reader, ReaderBuilder};

fn main() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();
    // Customize ReaderBuilder, builder, options
    builder
        .double_quote(false)
        .comment(Some(b'-'))
        .has_headers(false);
    let result = builder.from_path(file_name);
    // let result = Reader::from_path(file_name);

    // Handle failure on read operation
    if result.is_err() {
        println!("Failed to read CSV. File path may not exist or you don't have permissions.");
        std::process::exit(9);
    }

    // Perform file read
    let mut reader = result.unwrap();

    for record in reader.records() {
        // Avoid move operation | losing access to the record Result
        // by story the unwrapped result in a variable
        let data_value = record.unwrap();
        println!("{}", data_value.get(0).unwrap());
    }
}
