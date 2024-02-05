use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};

use phf::phf_map;

static DUCKY_INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "WINDOWS" => "0, MOD_GUI_LEFT",
    "GUI" => "0, MOD_GUI_LEFT",
 "APP" => "101",
"MENU" => "101",
"SHIFT" => "MOD_SHIFT_LEFT",
"ALT" => "MOD_ALT_LEFT",
"CONTROL" => "MOD_CONTROL_LEFT",
"CTRL" => "MOD_CONTROL_LEFT",
"DOWNARROW" => "81",
"DOWN" => "81",
"LEFTARROW" => "80",
"LEFT" => "80",
"RIGHTARROW" => "79",
"RIGHT" => "79",
"UPARROW" => "82",
"UP" => "82",
"BREAK" => "72",
"PAUSE" => "72",
"CAPSLOCK" => "57",
"DELETE" => "42",
"END" => "42",
"ESC" => "41",
"ESCAPE" => "41",
"HOME" => "74",
"NUMLOCK" => "83",
"PAGEUP" => "75",
"PAGEDOWN" => "78",
"PRINTSCREEN" => "70",
"SCROLLLOCK" => "71",
"SPACE" => "44",
"TAB" => "43",
"ENTER" => "KEY_ENTER",
"F1" => "KEY_F1",
"F2" => "KEY_F2",
"F3" => "KEY_F3",
"F4" => "KEY_F4",
"F5" => "KEY_F5",
"F6" => "KEY_F6",
"F7" => "KEY_F7",
"F8" => "KEY_F8",
"F9" => "KEY_F9",
"F10" => "KEY_F10",
"F11" => "KEY_F11",
"F12" => "KEY_F12",
"a" => "KEY_A",
"b" => "KEY_B",
"c" => "KEY_C",
"d" => "KEY_D",
"e" => "KEY_E",
"f" => "KEY_F",
"g" => "KEY_G",
"h" => "KEY_H",
"i" => "KEY_I",
"j" => "KEY_J",
"k" => "KEY_K",
"l" => "KEY_L",
"m" => "KEY_M",
"n" => "KEY_N",
"o" => "KEY_O",
"p" => "KEY_P",
"q" => "KEY_Q",
"r" => "KEY_R",
"s" => "KEY_S",
"t" => "KEY_T",
"u" => "KEY_U",
"v" => "KEY_V",
"w" => "KEY_W",
"x" => "KEY_X",
"y" => "KEY_Y",
"z" => "KEY_Z",
"A" => "KEY_A",
"B" => "KEY_B",
"C" => "KEY_C",
"D" => "KEY_D",
"E" => "KEY_E",
"F" => "KEY_F",
"G" => "KEY_G",
"H" => "KEY_H",
"I" => "KEY_I",
"J" => "KEY_J",
"K" => "KEY_K",
"L" => "KEY_L",
"M" => "KEY_M",
"N" => "KEY_N",
"O" => "KEY_O",
"P" => "KEY_P",
"Q" => "KEY_Q",
"R" => "KEY_R",
"S" => "KEY_S",
"T" => "KEY_T",
"U" => "KEY_U",
"V" => "KEY_V",
"W" => "KEY_W",
"X" => "KEY_X",
"Y" => "KEY_Y",
"Z" => "KEY_Z",
"1" => "KEY_1",
"2" => "KEY_2",
"3" => "KEY_3",
"4" => "KEY_4",
"5" => "KEY_5",
"6" => "KEY_6",
"7" => "KEY_7",
"8" => "KEY_8",
"9" => "KEY_9",
"0" => "KEY_0",
"!" => "30",
"\"" => "49",
"#" => "32",
"$" => "33",
"%" => "34",
"&" => "36",
"\'" => "52",
"(" => "38",
")" => "39",
"*" => "37",
"+" => "46",
"," => "54",
"-" => "45",
"." => "55",
"/" => "56",
":" => "51",
";" => "51",
"<" => "54",
"=" => "46",
">" => "55",
"?" => "56",
"@" => "31",
"[" => "47",
"]" => "48",
"^" => "35",
"_" => "45",
"`" => "53",
"{" => "47",
"}" => "48",
"|" => "49",
"~" => "53",
};

fn convertInstruction(line: &str) -> Option<&str> {
    return DUCKY_INSTRUCTIONS.get(line).cloned();
}

const C_HEADER: &str = "#include \"DigiKeyboard.h\"\n\nvoid setup() {}\n\nvoid loop() {\n";

const C_FOOTER: &str = "\n}";

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let m = Command::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::new("input file")
                .short('i')
                .long("input")
                .required(true),
        )
        .arg(Arg::new("output file").short('o').long("output file"))
        .get_matches();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(m.get_one::<String>("input file").unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if m.contains_id("output file") {
                // Write to file
            } else {
                // Write to STDOUT
            }
        }
    } else {
        println!("Invalid input file provided!");
    }
}
