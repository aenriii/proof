use super::{ttablegen::generateTruthTable, notation::LOGICAL_AND};



pub(crate) fn assignment1() {
    generateTruthTable(
        vec!["P".to_string(), "Q".to_string(), "R".to_string()],
        vec![
            (format!("P {} Q", LOGICAL_AND), Box::new(|x| x[0] && x[1])),
            (format!("P {} R", LOGICAL_AND), Box::new(|x| x[0] && x[2])),
            (format!("Q {} R", LOGICAL_AND), Box::new(|x| x[1] && x[2])),
            (format!("P {} Q {} R", LOGICAL_AND, LOGICAL_AND), Box::new(|x| x[0] && x[1] && x[2])),
            (format!("P {} (Q {} R)", LOGICAL_AND, LOGICAL_AND), Box::new(|x| x[0] && (x[1] && x[2]))),
            (format!("(P {} Q) {} R", LOGICAL_AND, LOGICAL_AND), Box::new(|x| (x[0] && x[1]) && x[2])),

        ]
    );
}