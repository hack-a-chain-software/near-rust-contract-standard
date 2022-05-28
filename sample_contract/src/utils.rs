/// Utilitarina functions not associated with any struct

pub fn create_struct_id(part_1: String, part_2: String) -> String {
    format!(part_1, "@", part_2)
}