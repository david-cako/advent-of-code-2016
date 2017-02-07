fn isReal(&str room) -> bool {
    let (name, rest) = room.split(char::is_numeric)[0].collect();
    let (number, checksum) = rest.split("[").collect();
    checksum.trim_right_matches("]");

