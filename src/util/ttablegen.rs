



type TruthTableCols = Vec<(TTHeader, TTEntry)>;
type TTHeader = String;
type TTEntry = Box<dyn Fn(Vec<bool>) -> bool>;

pub fn generate_truth_table(
    variables: Vec<String>,
    cols: TruthTableCols,
) {
    let mut v = variables.clone();
    v.extend(cols.iter().map(|x| x.0.clone()));
    let mut header_line = v  
        .iter()
        .map(|x| format!(" {} ", x))
        .collect::<Vec<String>>()
        .join("|");

    header_line = format!("|{}|", header_line);

    let header_line_len = header_line.len();
    let mut header_line = format!("{}\n", header_line);
    header_line.push_str(&format!("|{}|", "-".repeat(header_line_len - (2 * (variables.len() + cols.len())) - 2)));
    header_line.push_str("\n");
    let mut table = header_line;

    let rows = 2usize.pow(variables.len() as u32);
    let mut row = 0;
    while row < rows {
        let mut row_str = String::new();
        let mut col = 0;
        while col < variables.len() {
            let val = row & (1 << col) != 0;
            // row_str.push_str(&format!("| {} ", val));
            if val {
                row_str.push_str(&format!("| {} ", "T"));
            } else {
                row_str.push_str(&format!("| {} ", "F"));
            }
            col += 1;
        }
        for col_f in cols.iter() {
            let val = col_f.1(
                variables
                    .iter()
                    .map(|x| row & (1 << variables.iter().position(|y| y == x).unwrap()) != 0)
                    .collect::<Vec<bool>>(),
            );

            row_str.push_str(&format!("|"));
            let mut s = format!(
                "{}{}{}",
                " ".repeat(
                    {
                        let len_orig = col_f.0.len() - 2;
                        // println!("{}", len_orig);
                        // println!("{}", (len_orig - 1).div_floor(2) + 1);
                        (len_orig - 1).div_floor(2) + 1
                    }
                ),
                if val {
                    "T"
                } else {
                    "F"
                },
                " ".repeat(
                    {
                        let len_orig = col_f.0.len() - 2;
                        // println!("{}", len_orig);
                        (len_orig - 1).div_floor(2) + 1 
                    }
                ),
            );
            if s.len() > 8 {
                s = s[1..s.len() - 1].to_string();
            }
            row_str.push_str(&s);
        }
        row_str.push_str("|\n");
        table.push_str(&row_str);
        row += 1;
    }
    println!("{}", table);


}