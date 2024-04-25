pub fn get_diamond(c: char) -> Vec<String> {
    // Determine the size of the diamond
    let size = (c as u8 - b'A' + 1) as usize;
    // Create an empty vector to hold the diamond rows
    let mut diamond = Vec::with_capacity(2 * size - 1);
    
    // Generate top half of the diamond
    for i in 0..size {
        let mut row = String::new();
        // Calculate the number of spaces needed
        let spaces = size - i - 1;
        // Fill the row with spaces
        row.push_str(&" ".repeat(spaces));
        // Append the left part of the row (characters in ascending order)
        row.push((b'A' + i as u8) as char);
        // Append the middle part of the row
        if i > 0 {
            row.push_str(&" ".repeat(2 * i - 1));
            row.push((b'A' + i as u8) as char);
        }
        // Append the right part of the row
        if i < size - 1 {
            row.push_str(&" ".repeat(spaces));
        }
        // Add the row to the diamond
        diamond.push(row);
    }
    
    // Generate bottom half of the diamond
    for i in (0..size - 1).rev() {
        let mut row = String::new();
        // Calculate the number of spaces needed
        let spaces = size - i - 1;
        // Fill the row with spaces
        row.push_str(&" ".repeat(spaces));
        // Append the left part of the row (characters in ascending order)
        row.push((b'A' + i as u8) as char);
        // Append the middle part of the row
        if i > 0 {
            row.push_str(&" ".repeat(2 * i - 1));
            row.push((b'A' + i as u8) as char);
        }
        // Append the right part of the row
        if i < size - 1 {
            row.push_str(&" ".repeat(spaces));
        }
        // Add the row to the diamond
        diamond.push(row);
    }
    
    diamond
}
