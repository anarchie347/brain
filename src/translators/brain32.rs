pub fn translate(source: Vec<CodeBlock>) -> String {
    let mut full = vec!['>'; 4]; //wraps code in arrows to go to D0 to start,
    full.append(&mut source.iter().flat_map(translate_block).collect());
    full.append(&mut vec!['<'; 4]);
    full.iter().collect()
}

pub fn parse(source: Vec<(Mode, String)>) -> Vec<CodeBlock> {
    source
        .iter()
        .map(|(mode, section)| CodeBlock {
            mode: *mode,
            code: parse_section(section),
        })
        .collect()
}

//starts in W0
const DATA_ADD_FULL: String = String::from(concat!(
    "[-]>+>+>+>+<<<<<" //zero W0, increment all D
    data_copy_to_working(0),
    "[>>>-" //if W0(=D0) != 0, undo D1 carry, end up in D1
    "<<<[-]" //return to W0 and set 0



));

macro_rules! ptr_W0_to_Dx {
    (0) => {
        "<"
    };
    (1) => {
        "<<"
    };
    (2) => {
        "<<<"
    };
    (3) => {
        "<<<<"
    };
}

macro_rules! ptr_Dx_to_W0 {
    (0) => {
        ">"
    };
    (1) => {
        ">>"
    };
    (2) => {
        ">>>"
    };
    (3) => {
        ">>>>"
    };
}

macro_rules! move_patterned {
    ($($x:expr), *) => {
        let mut move_code = String::from("[");
        $(
            let direction = if $x < 0 {'<'} else {'>'};
            result.push_str(direction.repeat(($x).abs()));
            result.push('+');
        )*

        $(
            let direction = if $x < 0 {'>'} else {'<'};
            result.push_str(direction.repeat(($x).abs()));
        )*
        result.push("-");
    };
}

//--------------The following assume the pointer starts in W0 and returns the pointer to W0

macro_rules! move_Dx_to_W0 {
    () => {};
}

macro_rules! move_W0_to_Dx {
    () => {};
}

macro_rules! copy_Dx_to_W0 {
    () => {};
}

macro_rules! copy_W0_to_Dx {
    () => {};
}
