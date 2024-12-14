use std::io;

type Rules = Vec<(usize, usize)>;
type Manual = Vec<usize>;

fn is_valid(rules: &Rules, manual: &Manual) -> bool {
    rules.iter().all(|(r1, r2)| {
        let r1_pos = manual.iter().position(|p| p == r1);
        let r2_pos = manual.iter().position(|p| p == r2);

        if let (Some(r1_pos), Some(r2_pos)) = (r1_pos, r2_pos) {
            r1_pos < r2_pos
        } else {
            true
        }
    })
}

fn main() {
    let input: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let separator = input.iter()
        .position(String::is_empty)
        .expect("no separator");

    let rules = &input[..separator];
    let manuals = &input[separator + 1..];

    let rules: Rules = rules.iter()
        .map(|s| s.split_once('|').expect("no rule separator"))
        .map(|(s1, s2)| {
            (s1.parse::<usize>().expect("not a number"),
             s2.parse::<usize>().expect("not a number"))
        })
        .collect();

    let manuals: Vec<Manual> = manuals.iter()
        .map(|s| s.split(","))
        .map(|pages| pages.map(|s| s.parse::<usize>().expect("not a number")).collect::<Vec<usize>>())
        .collect();

    let valid: Vec<_> = manuals.iter()
        .filter(|manual| is_valid(&rules, manual))
        .collect();

    let result: usize = valid.iter().map(|manual| manual[manual.len() / 2]).sum();

    println!("part 1: {result}");

    let invalid: Vec<_> = manuals.iter()
        .filter(|manual| !is_valid(&rules, manual))
        .cloned().collect();

    let mut result = 0;

    for mut manual in invalid {
        while !is_valid(&rules, &manual) {
            for (r1, r2) in &rules {
                let r1_pos = manual.iter().position(|p| p == r1);
                let r2_pos = manual.iter().position(|p| p == r2);

                if let (Some(r1_pos), Some(r2_pos)) = (r1_pos, r2_pos) {
                    if r1_pos > r2_pos {
                        let tmp = manual.remove(r2_pos);
                        manual.insert(r1_pos, tmp);

                        break;
                    }
                }
            }
        }

        result += manual[manual.len() / 2]
    }

    println!("part 2: {result}");
}
