use std::{cmp::max, collections::BTreeMap};

pub fn task1(lines: &[String]) -> usize {
    let mut lines = lines.iter();
    let mut itree: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (s, e) = line.split_once('-').expect("Input should be `start-end`");
        let s: usize = s.parse().unwrap();
        let e: usize = e.parse().unwrap();

        itree.entry(s).or_default().push(e);
    }

    let mut result = 0;
    while let Some(line) = lines.next() {
        let n = line.parse::<usize>().unwrap();

        let range = itree.range(..=n);
        'outer: for (_, ends) in range {
            for end in ends {
                if *end >= n {
                    result += 1;
                    break 'outer;
                }
            }
        }
    }

    result
}

pub fn task2(lines: &[String]) -> usize {
    let mut lines = lines.iter();
    let mut itree: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (s, e) = line.split_once('-').expect("Input should be `start-end`");
        let s: usize = s.parse().unwrap();
        let e: usize = e.parse().unwrap();

        itree.entry(s).or_default().push(e);
    }

    let mut result = 0;

    let mut last_end: Option<usize> = None;

    for (mut start, ends) in itree {
        let end = ends.iter().max().expect("ends cannot be empty");

        if let Some(last_end) = last_end {
            start = max(last_end, start);
        }
        if *end >= start {
            result += end - start + 1;
            last_end = Some(*end + 1);
        }
    }

    result
}
