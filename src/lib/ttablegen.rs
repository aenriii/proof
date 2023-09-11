use std::borrow::BorrowMut;



type TruthTableCols = Vec<(TTHeader, TTEntry)>;
type TTHeader = String;
type TTEntry = Box<dyn Fn(Vec<bool>) -> bool>;

pub fn generateTruthTable(
    variables: Vec<String>,
    cols: TruthTableCols,
) {
    let mut v = variables.clone();
    v.extend(cols.iter().map(|x| x.0.clone()));
    let mut headerLine = v  
        .iter()
        .map(|x| format!(" {} ", x))
        .collect::<Vec<String>>()
        .join("|");

    headerLine = format!("|{}|", headerLine);

    let mut headerLineLen = headerLine.len();
    let mut headerLine = format!("{}\n", headerLine);
    headerLine.push_str(&format!("|{}|", "-".repeat(headerLineLen - (2 * (variables.len() + cols.len())) - 2)));
    headerLine.push_str("\n");
    let mut table = headerLine;

    let mut rows = 2usize.pow(variables.len() as u32);
    let mut row = 0;
    while row < rows {
        let mut rowStr = String::new();
        let mut col = 0;
        while col < variables.len() {
            let mut val = row & (1 << col) != 0;
            // rowStr.push_str(&format!("| {} ", val));
            if val {
                rowStr.push_str(&format!("| {} ", "T"));
            } else {
                rowStr.push_str(&format!("| {} ", "F"));
            }
            col += 1;
        }
        for col_f in cols.iter() {
            let mut val = col_f.1(
                variables
                    .iter()
                    .map(|x| row & (1 << variables.iter().position(|y| y == x).unwrap()) != 0)
                    .collect::<Vec<bool>>(),
            );

            rowStr.push_str(&format!("|"));
            let mut s = format!(
                "{}{}{}",
                " ".repeat(
                    {
                        let len_orig = col_f.0.len() - 2;
                        // println!("{}", len_orig);
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
            rowStr.push_str(&s);
        }
        rowStr.push_str("|\n");
        table.push_str(&rowStr);
        row += 1;
    }
    println!("{}", table);


}