use std::cmp::{max, min};
use std::collections::{HashMap};
use std::ops::{Range};
use range_ext::intersect::Intersect;
use regex::Regex;
use crate::utils::read_lines;

pub(crate) fn day_15_1() {
    let now = std::time::Instant::now();
    let mut sensed_areas: HashMap<i32, Vec<Range<i32>>> = HashMap::new();
    println!("init : {:?}", now.elapsed());

    let dataset = parse_file("./data/input_15.txt");
    let key = 2000000;
    println!("Parsed data : {:?}", now.elapsed());


    dataset.clone().into_iter().enumerate().for_each(|(index, data)| {
        // println!("We are analyzing sensor {}/{}", index + 1, dataset.len());
        draw_sensor_v2(&mut sensed_areas, data[0], data[1], Some(key), true)
    });

    println!("run sensors : {:?}", now.elapsed());

    let mut area_list = sensed_areas.get(&key).unwrap().clone();
    area_list.sort_by(|a, b| a.start.cmp(&b.start));

    for index in (1..area_list.len()).rev() {
        if let Some(new_range) = merge_ranges(area_list.first().unwrap().clone(), area_list[index].clone()) {
            area_list.remove(index);
            area_list[0] = new_range;
        }
    }

    let areas = area_list.iter().map(|area| area.len()).sum::<usize>();
    println!("(V2) Number case where we cannot have a beacon is {:?}", areas);
    println!("find result : {:?}", now.elapsed());
}

pub(crate) fn day_15_2() {
    let now = std::time::Instant::now();
    let mut sensed_areas: HashMap<i32, Vec<Range<i32>>> = HashMap::new();
    println!("init : {:?}", now.elapsed());

    let dataset = parse_file("./data/input_15.txt");
    let min_key = 0;
    let max_key = 4000000;
    println!("Parsed data : {:?}", now.elapsed());


    dataset.clone().into_iter().enumerate().for_each(|(index, data)| {
        println!("We are analyzing sensor {}/{}", index + 1, dataset.len());
        draw_sensor_v2(&mut sensed_areas, data[0], data[1], None, false)
    });

    println!("run sensors : {:?}", now.elapsed());

    for (key, mut area_list) in sensed_areas {
        if key < min_key || key > max_key {
            continue;
        }

        for _ in 0..3 {
            area_list = bulk_merge_ranges(area_list);
        }

        area_list.sort_by(|a, b| a.start.cmp(&b.start));
        area_list.dedup();

        if area_list.len() > 1 {
            println!("Possible place at {} => {:?}", key, area_list);
            println!("x = {}, y = {}", area_list[0].end, key);
            let result = area_list[0].end as u128 * 4000000 as u128 + key as u128;
            println!("Tuning Frequency is {}", result);
        }
    }
}

fn draw_sensor_v2(sensed_areas: &mut HashMap<i32, Vec<Range<i32>>>, sensor: (i32, i32), beacon: (i32, i32), row_index: Option<i32>, remove_beacon: bool) {
    let (x1, y1) = sensor;
    let (x2, y2) = beacon;

    let x_length = if x1 > x2 { (x2..x1).len() } else { (x1..x2).len() } as i32;
    let y_length = if y1 > y2 { (y2..y1).len() } else { (y1..y2).len() } as i32;

    let scan_length = x_length + y_length;

    (0..=scan_length).for_each(|y| {
        let length = scan_length - y;

        let range = (x1 - length)..(x1 + length + 1);

        for row in [y1 - y, y1 + y] {
            if row_index.is_some_and(|row_index| row != row_index) {
                continue;
            }

            let mut range = range.clone();

            //remove beacons
            if remove_beacon && row == y2 {
                if range.start == x2 {
                    range.start += 1;
                }

                if range.end - 1 == x2 {
                    range.end -= 1;
                }
            }

            if let Some(mut sensed_area) = sensed_areas.get_mut(&row) {
                let mut replaced: Option<(usize, Range<i32>)> = None;
                //Search if we have 2 range who intersect
                for x in 0..sensed_area.len() {
                    if let Some(new_range) = merge_ranges(sensed_area[x].clone(), range.clone()) {
                        replaced = Some((x, new_range));
                        break;
                    }
                }

                if let Some(replaced) = replaced {
                    //We have merged a range
                    sensed_area[replaced.0] = replaced.1.clone();
                } else {
                    //We need to add a new range
                    sensed_area.push(range.clone());
                }
            } else {
                // Row does not exist, we need to create it
                sensed_areas.entry(row).or_insert(vec![range.clone()]);
            }
        }
    });
}

fn merge_ranges(range_a: Range<i32>, range_b: Range<i32>) -> Option<Range<i32>> {
    if (range_a.end >= range_b.start && range_a.end <= range_b.end) ||
        (range_b.end >= range_a.start && range_b.end <= range_a.end) {
        //We have an intersection
        return Some(min(range_a.start, range_b.start)..max(range_a.end, range_b.end));
    }

    return None;
}

fn bulk_merge_ranges(mut ranges : Vec<Range<i32>>) -> Vec<Range<i32>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let ranges_a = ranges.clone();
    let ranges_b = ranges.clone();

    let mut results : Vec<Range<i32>> = Vec::new();
    for mut range_a in ranges_a {
        for range_b in &ranges_b {
            if let Some(new_range) = merge_ranges(range_a.clone(), range_b.clone()) {
                range_a = new_range;
            }
        }

        results.push(range_a);
    }

    return results;
}

fn parse_file(filename: &str) -> Vec<Vec<(i32, i32)>> {
    let mut data: Vec<Vec<(i32, i32)>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        for line in lines {
            if let Ok(ip) = line {
                let captures = re.captures(&ip).unwrap();
                let sensor = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap());
                let beacon = (captures[3].parse::<i32>().unwrap(), captures[4].parse::<i32>().unwrap());
                data.push(vec![sensor, beacon]);
            }
        }
    }

    return data;
}
