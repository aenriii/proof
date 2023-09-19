
#![allow(dead_code)]
/*
 The characters here are based on the Notation section of https://cs.indstate.edu/wiki/index.php/Math_for_CS_Review
*/


type NotationChar = &'static str;

pub static SET_REAL_NUMBERS: NotationChar = "ℝ";
pub static SET_NATURAL_NUMBERS: NotationChar = "ℕ";
pub static SET_INTEGERS: NotationChar = "ℤ";
pub static SET_RATIONAL_NUMBERS: NotationChar = "ℚ";
pub static SET_COMPLEX_NUMBERS: NotationChar = "ℂ";
pub static PAIR_REAL_NUMBERS: NotationChar = "ℝ²";
pub static IMAGINARY_UNIT: NotationChar = "ⅈ";
pub static FLOOR: NotationChar = "⌊ %%1%% ⌋";
pub static CEILING: NotationChar = "⌈ %%1%% ⌉";
pub static CARDINALITY: NotationChar = "| %%1%% |";
pub static END_OF_PROOF: NotationChar = "∎";
pub static THERE_EXISTS: NotationChar = "∃";
pub static THERE_NOT_EXISTS: NotationChar = "∄";
pub static IS_ELEM_OF: NotationChar = "∈";
pub static IS_NOT_ELEM_OF: NotationChar = "∉";
pub static FOR_ALL: NotationChar = "∀";
pub static NOT: NotationChar = "¬";
pub static EMPTY_SET: NotationChar = "∅";
pub static PRODUCT: NotationChar = "∏";
pub static SUM: NotationChar = "∑";
pub static INFINITY: NotationChar = "∞";
pub static SUCH_THAT_DIVIDES: NotationChar = "∣";
pub static SUCH_THAT_DOES_NOT_DIVIDE: NotationChar = "∤";
pub static LOGICAL_AND: NotationChar = "∧";
pub static LOGICAL_OR: NotationChar = "∨";
pub static UNION: NotationChar = "∪";
pub static INTERSECTION: NotationChar = "∩";
pub static THEREFORE: NotationChar = "∴";
pub static IS_CONGRUENT_TO: NotationChar = "≅";
pub static IS_PROPER_SUBSET: NotationChar = "⊂";
pub static IS_NOT_PROPER_SUBSET: NotationChar = "⊄";
pub static IS_SUBSET_OR_EQ: NotationChar = "⊆";
pub static IS_NOT_SUBSET_OR_EQ: NotationChar = "⊈";
pub static EXCL_OR: NotationChar = "⊕";
pub static IS_APPROX: NotationChar = "~";
pub static INTEGERS_MOD_N: NotationChar = "ℤₙ";
pub static BASE_NATRL_LOG: NotationChar = "e";
pub static NATRL_LOG: NotationChar = "ln";
pub static FACTORIAL: NotationChar = "!";
pub static IMPLIES: NotationChar = "⇒";
pub static IF_AND_ONLY_IF: NotationChar = "⇔";
pub static SQRT: NotationChar = "√";
pub static SET: NotationChar = "{ %%1%% }";
pub static SET_DIFFERENCE: NotationChar = "-";
pub static CARTESIAN_PRODUCT: NotationChar = "×";
pub static EDGES_OF_GRAPH: NotationChar = "E";
pub static VERTICES_OF_GRAPH: NotationChar = "V";
pub static NUM_EDGES_OF_GRAPH: NotationChar = "m";
pub static NUM_VERTICES_OF_GRAPH: NotationChar = "n";
pub static DEGREE_OF_VERTEX: NotationChar = "deg";
pub static SUCH_THAT: NotationChar = "s.t.";
pub static NOT_EQ: NotationChar = "≠";