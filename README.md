<!-- # im just using this for things in cs you can ignore this if you want

## usage

this is a terminal-based application that generates latex files for
proofs based on the user's input. it's not really meant to be used
by anyone other than me but if you want to use it then go ahead

### planned user experience

- user inputs document title, aithor, and date
while not_stopped:
  - user inputs section/theorem/proof name
  - user inputs theorem statement
  - user inputs proof
  - application replaces `r#[NotationChars::id]` with `latexify(NotationChars.by_id($1))`

### example input

```.
$ proof

Enter the title of the document: "Example Document"
Enter the author of the document: "Jai"
Enter the date of the document: "2021-10-10"

Include a table of contents? [Y/n]:

Enter the name of section 1: "Example Section"

[Help: Entering two blank lines will stop the input process]
[Help: You can use latex syntax in your input]
Enter the text of section 1: \textbf{For all integers k, 2k+1 is odd.}

This can be proven 

```

## sources

i know i have to cite what external sources i use but
idk what that constitutes as so here's a list of
dependencies and why im using them

- strsim: for fuzzy string matching (in latex document generation)
- latex: for latex document generation -->

# cs203

im going rapidly insane
