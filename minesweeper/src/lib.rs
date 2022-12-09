const MINE: char = '*';

fn tile(r: usize, c: usize, field: &[&str]) -> Option<char> {
    field.get(r).and_then(|row| row.chars().nth(c))
}

fn tile_value(row: usize, col: usize, field: &[&str]) -> char {
    let mut mine_count = 0; 
    for r in row.saturating_sub(1)..=row + 1 {
        for c in col.saturating_sub(1)..=col + 1 {
            if tile(r, c, field) == Some( MINE ) {
                mine_count += 1;
            }
        }
    }
    if mine_count == 0 { ' ' } 
    else { char::from(48u8 + mine_count) }
} 

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate()
             .map(|(r, row)| {
                row.chars().enumerate()
                   .map(|(c, tile)| match tile {
                            '*' => MINE,
                             _  => tile_value(r, c, minefield)
                        }
                    ).collect()
             }).collect()
}
