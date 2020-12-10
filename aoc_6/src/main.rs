use std::collections::HashMap;
use util::res::Result;

struct GroupAnswers {
    num_people: usize,
    num_yeses_by_question: HashMap<char, usize>,
}

fn file_contents_to_group_answers(contents: &String) -> Vec<GroupAnswers> {
    let mut out = vec![];
    let mut curr_yeses_by_question: HashMap<char, usize> = HashMap::new();
    let mut people_in_group: usize = 0;

    // Add an extra blank line to the end of the lines so we don't have to do extra post-loop logic
    for line in contents.lines().chain(std::iter::once("")) {
        if line == "" {
            let num_yeses_by_question = curr_yeses_by_question.drain().collect();
            out.push(GroupAnswers{num_people: people_in_group, num_yeses_by_question: num_yeses_by_question});
            people_in_group = 0;
        } else {
            line.chars().for_each(|c| {
                let people_with_yes_entry = curr_yeses_by_question.entry(c).or_insert(0);
                *people_with_yes_entry += 1;
            });
            people_in_group += 1;
        }
    }

    out
}

fn part1(group_answers: &Vec<GroupAnswers>) {
    let sum = group_answers.iter().fold(0, |acc, ga| acc + ga.num_yeses_by_question.len());
    println!("[Part 1] Sum of # questions answered per group, across all groups: {}", sum);
}

fn part2(group_answers: &Vec<GroupAnswers>) {
    let sum = group_answers.iter().fold(0, |acc, ga| {
        acc + ga.num_yeses_by_question.iter().filter(|(_, &c)| c == ga.num_people).count()
    }); 
    println!("[Part 2] Sum of # questions answered by all in group, across all groups: {}", sum);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let group_answers = file_contents_to_group_answers(&contents);

    part1(&group_answers);
    part2(&group_answers);
    Ok(())
}