struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut db = [0; 26];
        magazine
            .into_bytes()
            .iter()
            .for_each(|&x| db[(x - 'a' as u8) as usize] += 1);
        for s in ransom_note.into_bytes().into_iter() {
            let idx = (s - 'a' as u8) as usize;
            if db[idx] == 0 {
                return false;
            }
            db[idx] -= 1;
        }
        true
    }
}
