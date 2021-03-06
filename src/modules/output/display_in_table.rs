use term_table::row::Row;
use term_table::table_cell::TableCell;

// Public function
// Gets data in nested vector of string form and displays it in a table cell
pub fn display_data(data: &Vec<Vec<String>>) {
    let mut table = term_table::Table::new();
    table.max_column_width = 28;
    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(
            "\x1b[1;32mTask ID\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mTask Name\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mStart Date\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mStart Time\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mEnd Date\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mEnd Time\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mImportant\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            "\x1b[1;32mCompleted\x1b[0m",
            1,
            term_table::table_cell::Alignment::Center,
        ),
    ]));
    for i in 0..data.len() {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(
                &data[i][0],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][1],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][2],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][3],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][4],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][5],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][6],
                1,
                term_table::table_cell::Alignment::Center,
            ),
            TableCell::new_with_alignment(
                &data[i][7],
                1,
                term_table::table_cell::Alignment::Center,
            ),
        ]));
    }
    println!("{}", table.render());
}
