use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;
use std::mem::replace;
use std::io::{read_to_string, BufReader};

#[derive(Debug, Clone)]
struct Part {
    line: usize,
    column: usize,
    associated_pns: Vec<PartNumber>,
    ptype: char,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Part at line {}, col {} with associated Part nums {:?}>", self.line, self.column, self.associated_pns)
    }
}

#[derive(Debug, Clone, Copy)]
struct PartNumber {
    line: usize,
    column: usize,
    digits: u8,
    val: u32,
}

impl PartNumber {
    fn in_vicinity(&self, part: &Part) -> bool {
        let minline:usize = if self.line > 0 { self.line - 1 } else { 0 };
        let maxline:usize = self.line + 1;
        let mincol:usize = if self.column > 0 { self.column - 1 } else { 0 };
        let maxcol:usize = self.column + self.digits as usize + 1;
        minline <= part.line && part.line <= maxline && mincol <= part.column && part.column < maxcol
    }
}

impl PartNumber {
    fn new(s: &str, line: usize, col: usize) -> Result<Self, ParseError> {
        if let Ok(val) = s.parse::<u32>() {
            Ok(PartNumber {
                line,
                column: col,
                digits: s.len() as u8,
                val,
            })
        } else {
            Err(ParseError::NumberNotAnInteger(line, col))
        }
    }
}

impl Display for PartNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

struct EngineSchematic {
    parts: Vec<Part>
}

impl EngineSchematic {
    fn new() -> Self {
        EngineSchematic { parts: Vec::new() }
    }
    fn sum_of_part_nums(&self) -> usize {
        let mut part_num_sum = 0 as usize;
        for part in self.parts.iter() {
            for pn in part.associated_pns.iter() {
                part_num_sum = part_num_sum + pn.val as usize;
            }
        };
        part_num_sum
    }
    fn gear_ratios(&self) -> usize {
        let mut total_ratios: usize = 0;
        for g in self.parts.iter().filter(|p| {p.ptype == '*' && p.associated_pns.len() == 2}) {
            total_ratios += g.associated_pns[0].val as usize * g.associated_pns[1].val as usize
        }
        total_ratios
    }
}

impl FromStr for EngineSchematic {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut es = EngineSchematic::new();
        let mut sw = ScanWindow::new();
        for (lnr, line) in s.lines().enumerate() {
            let mut processed_parts = sw.process_line(line, lnr)?;
            es.parts.append(&mut processed_parts)
        }
        es.parts.append(&mut sw.parts_in_last_line);
        Ok(es)
    }
}

struct ScanWindow {
    parts_in_last_line: Vec<Part>,
    pn_in_last_line: Vec<PartNumber>,
    state: ScanState,
}

enum ScanState {
    Normal,
    PartNum(usize),
}

enum ParseError {
    NumberNotAnInteger(usize, usize),
    UnexpectedSymbolFound(usize, usize),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumberNotAnInteger(line, col) => write!(f, "The number at line {} col {} is not an integer", line, col), 
            Self::UnexpectedSymbolFound(line, col) => write!(f, "The symbol at line {}, col {} is not expected", line, col)
        }
    }
}

impl ScanWindow {
    fn new() -> Self {
        ScanWindow { parts_in_last_line: Vec::new(), pn_in_last_line: Vec::new(), state: ScanState::Normal}
    }

    fn process_line(&mut self, line: &str, line_nr: usize) -> Result<Vec<Part>, ParseError> {
        let mut parts_in_line: Vec<Part> = Vec::new();
        let mut part_numbers_in_line: Vec<PartNumber> = Vec::new();
        // Add all the part numbers and parts to the current scan window
        for (col, c) in line.chars().enumerate() {
            match (c, &self.state) {
                ('.', ScanState::Normal) => {
                },
                (t, ScanState::Normal) if t.is_numeric() => {
                    self.state = ScanState::PartNum(col);
                },
                (t, ScanState::Normal) if !t.is_numeric() && t != '.'  => {
                    parts_in_line.push(Part { line: line_nr, column: col, associated_pns: Vec::new(), ptype: t});
                },
                (t, ScanState::PartNum(numstart)) if !t.is_numeric() && t == '.' => {
                    part_numbers_in_line.push(PartNumber::new(&line[*numstart..col], line_nr, *numstart)?);
                    self.state = ScanState::Normal;
                },
                (t, ScanState::PartNum(numstart)) if !t.is_numeric() && t != '.' => {
                    part_numbers_in_line.push(PartNumber::new(&line[*numstart..col], line_nr, *numstart)?);
                    parts_in_line.push(Part { line: line_nr, column: col, associated_pns: Vec::new(), ptype: t});
                    self.state = ScanState::Normal;
                },
                (t, ScanState::PartNum(_)) if t.is_numeric() => {
                    continue;
                }
                _ => return Err(ParseError::UnexpectedSymbolFound(line_nr, col))
            }
        }
        match self.state {
            ScanState::PartNum(numstart) => {
                part_numbers_in_line.push(PartNumber::new(&line[numstart..line.len()], line_nr, numstart)?);
                self.state = ScanState::Normal;
            },
            _ => {}
        }
        // see if we can find any part numbers on the current line that match to parts on the
        // current line
        for part_number in part_numbers_in_line.iter() {
            for part in parts_in_line.iter_mut() {
                if part_number.in_vicinity(part) {
                    part.associated_pns.push(*part_number);
                }
            }
        }
        // see if we can find any part numbers in the previous line that match with parts on the
        // current line
        for part_number in self.pn_in_last_line.iter() {
            for part in parts_in_line.iter_mut() {
                if part_number.in_vicinity(part) {
                    part.associated_pns.push(*part_number);
                }
            }
        }
        // see if we can find any parts on the previous line that match with parts on the current
        // line
        for part_number in part_numbers_in_line.iter() {
            for part in self.parts_in_last_line.iter_mut() {
                if part_number.in_vicinity(part) {
                    part.associated_pns.push(*part_number);
                }
            }
        }
        println!("Parts in prev line {}: {:?}", line_nr , self.parts_in_last_line);
        // we have assigned the part numbers of the last line to the parts in the current
        // line and assigned the part numbers of this line with the parts in the last line
        let _ = replace(&mut self.pn_in_last_line, part_numbers_in_line);
        Ok(replace(&mut self.parts_in_last_line, parts_in_line))
    }
}

pub fn solve_day_3(freader: BufReader<File>, _: bool) {
    if let Ok(engine_schematic) = read_to_string(freader) {
        match engine_schematic.parse::<EngineSchematic>() {
            Err(e) => println!("Encountered Error while parsing the Document: {}", e),
            Ok(schematic) => {
                println!("Total Part number Sum: {}", schematic.sum_of_part_nums());
                println!("Sum of Gear Ratios: {}", schematic.gear_ratios());
            }
        } 
    } else {
        println!("Unable to read string from buffer");
    }
}
