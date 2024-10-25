use crate::file_handler::{read_from_file, write_to_file};

const NB_HIGHSCORES_SAVED: usize = 5;

pub fn load_scores_and_lines(sep: Option<&str>) -> Option<(Vec<u32>, Vec<u32>)> {
    let sep = add_default_separator(sep);
    if let Ok(scores_and_lines) = read_from_file("assets/scores.txt") {
        let mut lines = scores_and_lines.splitn(2, "\n")
                                    .map(|line| convert_line_to_vector(line, sep))
                                    .collect::<Vec<_>>();
        if lines.len() == 2 {
            Some((lines.pop().unwrap(), lines.pop().unwrap()))
        }
        else {
            None
        }
    }
    else {
        None
    }
}

pub fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32], sep: Option<&str>) -> bool {
    let sep = add_default_separator(sep);
    let highscores_as_string = convert_to_string_line(highscores, sep);
    let number_of_lines_as_string = convert_to_string_line(number_of_lines, sep);
    write_to_file(format!("{}\n{}\n", highscores_as_string, number_of_lines_as_string).as_str(), "assets/scores.txt").is_ok()
}

pub fn update_new_achievement_vec(highscores: &mut Vec<u32>, new_value: u32) -> bool {
    if highscores.len() < NB_HIGHSCORES_SAVED {
        highscores.push(new_value);
        highscores.sort_by(|a, b| b.cmp(a));
        return true;
    }
    else {
        if highscores.iter().any(|x| *x < new_value) {
            highscores.pop();
            highscores.push(new_value);
            highscores.sort_by(|a, b| b.cmp(a));
            return  true;
        }

        return false;
    }
}

fn add_default_separator(sep: Option<&str>) -> &str {
    let sep = match sep {
        Some(value) => value,
        None => " "
    };
    sep
}

fn convert_line_to_vector(line: &str, sep: &str) -> Vec<u32> {
    line.split(sep)
        .map(|value| value.trim())
        .filter(|value| value.parse::<u32>().is_ok())
        .map(|value| value.parse::<u32>().unwrap())
        //.filter_map(|value| value.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn convert_to_string_line<T:ToString>(values: &[T], sep: &str) -> String {
    values.iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
