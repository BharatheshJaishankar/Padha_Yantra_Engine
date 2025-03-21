use std::collections::HashMap;
#[warn(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct Definition {
    word: String,
    definition: Vec<Define>,
}

struct Define {
    entry: String,
    typ: String,
}

fn read_file_range(file_path: &str, start: u32, end: u32) -> Vec<String> {
    let file = match file_path {
      "word_index.txt" => include_str!("../word_index.txt"),
      "definition.txt" => include_str!("../definition.txt"),
      _ => panic!("file not found"),
    };
    let mut contents: Vec<String> = Vec::new();
    let reader = BufReader::new(file.as_bytes());
    let end_computed: u32 = (end - start) + 1;
    for line in reader
        .lines()
        .skip((start - 1) as usize)
        .take(end_computed as usize)
    {
        contents.push(line.expect("Unable to read line"));
    }
    contents
}

lazy_static::lazy_static! {
  static ref KANNADA_HEADS: HashMap<char, char> = {
      let mut map = HashMap::new();
      map.insert('ಅ', 'ಅ');
      map.insert('ಆ', 'ಆ');
      map.insert('ಇ', 'ಇ');
      map.insert('ಈ', 'ಈ');
      map.insert('ಉ', 'ಉ');
      map.insert('ಊ', 'ಊ');
      map.insert('ಋ', 'ಋ');
      map.insert('ೠ', 'ೠ');
      map.insert('ಎ', 'ಎ');
      map.insert('ಏ', 'ಏ');
      map.insert('ಐ', 'ಐ');
      map.insert('ಒ', 'ಒ');
      map.insert('ಓ', 'ಓ');
      map.insert('ಔ', 'ಔ');
      // Consonants
      map.insert('ಕ', 'ಕ');
      map.insert('ಖ', 'ಖ');
      map.insert('ಗ', 'ಗ');
      map.insert('ಘ', 'ಘ');
      map.insert('ಙ', 'ಙ');
      map.insert('ಚ', 'ಚ');
      map.insert('ಛ', 'ಛ');
      map.insert('ಜ', 'ಜ');
      map.insert('ಝ', 'ಝ');
      map.insert('ಞ', 'ಞ');
      map.insert('ಟ', 'ಟ');
      map.insert('ಠ', 'ಠ');
      map.insert('ಡ', 'ಡ');
      map.insert('ಢ', 'ಢ');
      map.insert('ಣ', 'ಣ');
      map.insert('ತ', 'ತ');
      map.insert('ಥ', 'ಥ');
      map.insert('ದ', 'ದ');
      map.insert('ಧ', 'ಧ');
      map.insert('ನ', 'ನ');
      map.insert('ಪ', 'ಪ');
      map.insert('ಫ', 'ಫ');
      map.insert('ಬ', 'ಬ');
      map.insert('ಭ', 'ಭ');
      map.insert('ಮ', 'ಮ');
      map.insert('ಯ', 'ಯ');
      map.insert('ರ', 'ರ');
      map.insert('ಱ', 'ಱ');
      map.insert('ಲ', 'ಲ');
      map.insert('ೞ', 'ೞ');
      map.insert('ವ', 'ವ');
      map.insert('ಶ', 'ಶ');
      map.insert('ಷ', 'ಷ');
      map.insert('ಸ', 'ಸ');
      map.insert('ಹ', 'ಹ');
      map.insert('ಳ', 'ಳ');
      map
  };
}

fn get_head(word: &str) -> Option<char> {
    word.chars()
        .next()
        .and_then(|c| KANNADA_HEADS.get(&c).cloned())
}

pub fn get_definition(word: &str) -> Result<Definition, Error> {
    let head_result = get_head(word).map(|h| h.to_string());
    let head = match head_result {
        Some(h) => h,
        None => return Err(ErrorKind::NotFound.into()),
    };
    let helper_file = String::from(
        "ಅ:1:3883
ಆ:3884:3884
ಅ:3885:9326
ಆ:9327:9327
ಅ:9328:10192
ಅ:10194:11743
ಆ:11744:11744
ಅ:11745:13225
ಆ:13226:13226
ಅ:13227:13237
ಆ:13238:13238
ಅ:13239:15083
ಆ:15084:15107
ಅ:15108:15109
ಆ:15110:15381
ಅ:15382:15385
ಆ:15386:15504
ಅ:15505:15505
ಆ:15506:15840
ಅ:15841:15841
ಆ:15842:16518
ಅ:16519:16519
ಆ:16520:16842
ಅ:16843:16843
ಆ:16844:17382
ಅ:17383:17383
ಆ:17384:18489
ಅ:18490:18490
ಆ:18491:19043
ಇ:19044:21107
ಈ:21108:21467
ಉ:21468:25449
ಊ:25450:25871
ಋ:25872:26022
ಎ:26023:28022
ಏ:28023:28775
ಐ:28776:29019
ಒ:29020:31023
ಓ:31024:31497
ಔ:31498:31636
ಕ:31637:33789
ನ:33790:33790
ಕ:33791:52831
ಖ:52832:53804
ಗ:53805:60573
ಘ:60574:61102
ಙ:61103:61105
ಚ:61106:65533
ಛ:65534:65819
ಜ:65820:69613
ಝ:69614:69853
ಞ:69854:69855
ಟ:69856:70205
ಠ:70206:70363
ಡ:70364:71036
ಢ:71037:71055
ಡ:71056:71056
ಢ:71057:71133
ಣ:71134:71139
ತ:71140:78431
ಥ:78432:78521
ದ:78522:83653
ಧ:83654:83984
ದ:83985:83985
ಧ:83986:84210
ದ:84211:84211
ಧ:84212:84282
ದ:84283:84283
ಧ:84284:84728
ನ:84729:85598
ನ:85600:86697
ನ:86699:87625
ನ:87627:92958
ಪ:92959:93022
ಪ:93024:94724
ಫ:94725:94725
ಪ:94726:96894
ಪ:96896:105553
ಫ:105554:105920
ಬ:105921:113354
ಭ:113355:115117
ಮ:115118:115601
ಮ:115603:116036
ಮ:116038:117204
ವ:117205:117205
ಮ:117206:118585
ಮ:118587:119621
ಮ:119623:119674
ಮ:119676:119686
ಮ:119688:125224
ಮ:125226:125452
ಯ:125453:126162
ರ:126163:128511
ಲ:128512:129195
ಲ:129197:129858
ಲ:129860:130083
ವ:130084:132054
ವ:132056:136156
ಶ:136157:136867
ಶ:136869:138852
ಷ:138853:139035
ಸ:139036:141066
ಸ:141068:143570
ಸ:143572:144351
ಸ:144353:149185
ಸ:149187:149826
ಸ:149828:150227
ಹ:150228:151510
ಹ:151512:155774
ಹ:155776:156585
ಳ:156586:156592
ಱ:156593:156667
ೞ:156668:156672
",
    );
    let mut find_head: Vec<String> = Vec::new();
    for line in helper_file.lines() {
        if line.contains(&head) {
            find_head.push(line.to_string());
        }
    }

    // Get Word
    let mut word_find: Vec<String> = Vec::new();
    for line in find_head {
        let index = line.split(":").collect::<Vec<&str>>();
        let start: u32 = index[1].parse().unwrap();
        let end: u32 = index[2].parse().unwrap();
        let get_range = read_file_range("word_index.txt", start, end);
        for line in get_range {
            if line.contains(word) {
                word_find.push(line.to_string());
            }
        }
    }
    // Get Definition
    let mut definition_find: Vec<String> = Vec::new();
    for line in word_find {
        let index = line.split(":").collect::<Vec<&str>>();
        let start: u32 = index[1].parse().unwrap();
        let end: u32 = index[2].parse().unwrap();
        let get_range = read_file_range("definition.txt", start, end);
        for line in get_range {
            definition_find.push(line.to_string());
        }
    }

    let mut define_final: Vec<Define> = Vec::new();
    for define in definition_find {
        let typ = match define.chars().next().unwrap() {
            '0' => "Noun",
            '1' => "Verb",
            '2' => "Adjective",
            '3' => "Adverb",
            '4' => "Preposition",
            '5' => "Interjection",
            '6' => "Independent Clause",
            '7' => "Conjunction",
            '8' => "None",
            '9' => "Sentence",
            '=' => "Pronoun",
            '|' => "Suffix",
            '_' => "Prepositional Phrase",
            '^' => "Pronounciation",
            '~' => "Prefix",
            '%' => "ವ.",
            _ => "None",
        };
        define_final.push(Define {
            entry: define[1..].trim_end().to_string(),
            typ: typ.to_string(),
        });
    }

    Ok(Definition {
        word: word.to_string(),
        definition: define_final,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let test_1: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
        let test_2: Vec<String> = vec!["Is".to_string()];
        let test_3: Vec<String> = vec!["Hello".to_string()];

        assert_eq!(read_file_range("Test.txt", 1, 2), test_1);
        assert_eq!(read_file_range("Test.txt", 4, 4), test_2);
        assert_eq!(read_file_range("Test.txt", 1, 1), test_3);
    }
    #[test]
    fn test_2() {
        let word_1 = "ಅಕಾರಣಿ";
        let word_2 = "ನಿಮ್ಮ";
        assert_eq!(get_head(word_1), Some('ಅ'));
        assert_eq!(get_head(word_2), Some('ನ'));
    }
    #[test]
    fn log() {
        let df = get_definition("ಅಕಶೇರು").unwrap();
        assert_eq!(df.word, "ಅಕಶೇರು".to_string());
        assert_eq!(
            df.definition[0].entry,
            "(zool.) having no backbone or a vertebral column; invertebrate.".to_string()
        );
        assert_eq!(df.definition[0].typ, "Adjective".to_string());
    }
}
