use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ResourceType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for ResourceType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "seed" => Ok(ResourceType::Seed),
            "soil" => Ok(ResourceType::Soil),
            "fertilizer" => Ok(ResourceType::Fertilizer),
            "water" => Ok(ResourceType::Water),
            "light" => Ok(ResourceType::Light),
            "temperature" => Ok(ResourceType::Temperature),
            "humidity" => Ok(ResourceType::Humidity),
            "location" => Ok(ResourceType::Location),
            _ => Err("Unable to parse unknown resource type."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ResourceValue {
    id: u64,
    res_type: ResourceType,
}

struct ResourceMapping {
    destination: ResourceType,
    source: ResourceType,
    mapping: Vec<(Range<u64>, Range<u64>)>,
}

impl ResourceMapping {
    fn map(&self, value: ResourceValue, inverse: bool) -> Result<ResourceValue, &str> {
        if (!inverse && (value.res_type != self.source))
            || (inverse && (value.res_type != self.destination))
        {
            return Err("Invalid source type");
        }

        if let Some(r) = self.mapping.iter().find(|range| {
            if inverse {
                range.0.contains(&value.id)
            } else {
                range.1.contains(&value.id)
            }
        }) {
            let range_src = if inverse { &r.0 } else { &r.1 };
            let range_dst = if inverse { &r.1 } else { &r.0 };
            Ok(ResourceValue {
                id: range_dst.start + (value.id - range_src.start),
                res_type: if inverse {
                    self.source
                } else {
                    self.destination
                },
            })
        } else {
            Ok(ResourceValue {
                id: value.id,
                res_type: if inverse {
                    self.source
                } else {
                    self.destination
                },
            })
        }
    }
}

fn parse_seeds(input: &Vec<String>) -> Vec<ResourceValue> {
    input
        .iter()
        .take(1)
        .flat_map(|line| {
            line.trim_start_matches(|c: char| !c.is_whitespace())
                .split_whitespace()
        })
        .filter_map(|seed_id| {
            Some(ResourceValue {
                id: seed_id.parse().ok()?,
                res_type: ResourceType::Seed,
            })
        })
        .collect()
}

fn parse_resource_mappings(input: &Vec<String>) -> Vec<ResourceMapping> {
    input
        .iter()
        .skip(1)
        .filter(|line| !line.is_empty())
        .fold(Vec::new(), |mut resources, line| {
            if line.ends_with("map:") {
                if let Some((l_type, r_type)) =
                    line.trim().trim_end_matches(" map:").split_once("-to-")
                {
                    let source_type = ResourceType::try_from(l_type).unwrap();
                    let dest_type = ResourceType::try_from(r_type).unwrap();

                    resources.push(ResourceMapping {
                        source: source_type,
                        destination: dest_type,
                        mapping: Vec::new(),
                    });
                }
            } else {
                let mut ranges_iter = line
                    .trim()
                    .split_whitespace()
                    .filter_map(|l| l.parse::<u64>().ok());
                let (dst_start, src_start, len) = (
                    &ranges_iter.next().unwrap(),
                    &ranges_iter.next().unwrap(),
                    &ranges_iter.next().unwrap(),
                );

                if let Some(res_map) = resources.last_mut() {
                    res_map
                        .mapping
                        .push((*dst_start..(dst_start + len), *src_start..(src_start + len)));
                }
            }
            resources
        })
}

fn get_mapping(
    res_map: &[ResourceMapping],
    input_val: &ResourceValue,
    target_type: ResourceType,
) -> ResourceValue {
    let mut cur_val: ResourceValue = input_val.clone();

    while let Some(mapping) = res_map.iter().find(|map| map.source == cur_val.res_type) {
        cur_val = mapping.map(cur_val, false).unwrap();
        if cur_val.res_type == target_type {
            break;
        }
    }
    cur_val
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day5").unwrap());
    let input_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let seeds = parse_seeds(&input_lines);
    let resource_maps = parse_resource_mappings(&input_lines);
    let location_min: u64 = seeds
        .iter()
        .map(|s| get_mapping(&resource_maps, s, ResourceType::Location).id)
        .min()
        .unwrap_or_default();
    println!("Min location id values: {}", location_min);

    //let location_min_range = seeds
    //    .iter()
    //    .step_by(2)
    //    .zip(seeds.iter().skip(1).step_by(2))
    //    .flat_map(|(r_start, num_elems)| r_start.id..(r_start.id + num_elems.id))
    //    .map(|i| ResourceValue {
    //        id: i,
    //        res_type: ResourceType::Seed,
    //    })
    //    .map(|s| get_mapping(&resource_maps, &s, ResourceType::Location).id)
    //    .min()
    //    .unwrap_or_default();
    //println!("Part 2 Min location id values: {}", location_min_range);
}
