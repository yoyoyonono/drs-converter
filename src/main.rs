use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoteEvent {
    LeftStep {
        lane: u8,
        width: u8,
    },
    RightStep {
        lane: u8,
        width: u8,
    },
    Jump,
    Down,
    LeftHoldStart {
        id: u8,
        lane: u8,
        width: u8,
    },
    RightHoldStart {
        id: u8,
        lane: u8,
        width: u8,
    },
    SlideWaypoint {
        id: u8,
        lane: u8,
        width: u8,
    },
    SlideEnd {
        id: u8,
        lane: u8,
        width: u8,
    },
    SimpleSkidWaypoint {
        id: u8,
        lane: u8,
        width: u8,
    },
    ComplexSkidWaypoint {
        id: u8,
        lane_start: u8,
        width_start: u8,
        lane_end: u8,
        width_end: u8,
    },
    SimpleSkidEnd {
        id: u8,
        lane: u8,
        width: u8,
    },
    ComplexSkidEnd {
        id: u8,
        lane_start: u8,
        width_start: u8,
        lane_end: u8,
        width_end: u8,
    },
}

impl NoteEvent {
    fn from_string(input_string: String) -> NoteEvent {
        let chars: Vec<char> = input_string.chars().collect();
        match chars[0] {
            '0' => NoteEvent::LeftStep {
                lane: u8::from_str_radix(&chars[1].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[2].to_string(), 16).unwrap() + 1,
            },
            '1' => NoteEvent::RightStep {
                lane: u8::from_str_radix(&chars[1].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[2].to_string(), 16).unwrap() + 1,
            },
            '2' => NoteEvent::Jump,
            '3' => NoteEvent::Down,
            '4' => NoteEvent::LeftHoldStart {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            '5' => NoteEvent::RightHoldStart {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            '6' => NoteEvent::SlideWaypoint {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            '7' => NoteEvent::SlideEnd {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            '8' => NoteEvent::SimpleSkidWaypoint {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            '9' => NoteEvent::ComplexSkidWaypoint {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane_start: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width_start: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
                lane_end: u8::from_str_radix(&chars[4].to_string(), 16).unwrap(),
                width_end: u8::from_str_radix(&chars[5].to_string(), 16).unwrap() + 1,
            },
            'A' => NoteEvent::SimpleSkidEnd {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
            },
            'B' => NoteEvent::ComplexSkidEnd {
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(),
                lane_start: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(),
                width_start: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
                lane_end: u8::from_str_radix(&chars[4].to_string(), 16).unwrap(),
                width_end: u8::from_str_radix(&chars[5].to_string(), 16).unwrap() + 1,
            },
            _ => panic!(),
        }
    }
}

enum LongPoint {
    Normal {
        point_time: u32,
        pos_left: u32,
        pos_right: u32,
    },
    SkidComplex {
        point_time: u32,
        pos_left_start: u32,
        pos_right_start: u32,
        pos_left_end: u32,
        pos_right_end: u32,
    },
    SkidSimple {
        point_time: u32,
        pos_left: u32,
        pos_right: u32,
    },
}

#[derive(Debug)]
struct Measure {
    ticks: Vec<Vec<NoteEvent>>,
}

impl Measure {
    fn new() -> Measure {
        Measure {
            ticks: vec![Vec::<NoteEvent>::new(); 192],
        }
    }
}

fn measure_tick_to_ms(measure: u32, tick: u32, bpm: u32) -> u32 {
    let ms_per_beat = 60000 / bpm;
    let ms_per_measure = ms_per_beat * 4;
    let ticks_per_measure = 192;
    let ms_per_tick = ms_per_measure as f32 / ticks_per_measure as f32;
    (measure as f32 * ms_per_measure as f32 + tick as f32 * ms_per_tick) as u32
}

fn ms_to_dt(ms: u32, bpm: u32) -> u32 {
    (ms as f32 * 0.008 * bpm as f32) as u32
}

fn add_s32_element(xml: &mut XMLElement, name: &str, value: u32) {
    let mut element = XMLElement::new(name);
    element.add_attribute("__type", "s32");
    element.add_text(value.to_string().into()).unwrap();
    xml.add_child(element).unwrap();
}

fn add_s64_element(xml: &mut XMLElement, name: &str, value: u64) {
    let mut element = XMLElement::new(name);
    element.add_attribute("__type", "s64");
    element.add_text(value.to_string().into()).unwrap();
    xml.add_child(element).unwrap();
}

fn xml_boilerplate(bpm: u32) -> XMLElement {
    let mut xml = XMLElement::new("data");

    add_s32_element(&mut xml, "seq_version", 8);

    let mut info = XMLElement::new("info");

    add_s32_element(&mut info, "tick", 480);

    let mut bpm_info = XMLElement::new("bpm_info");
    let mut bpm_ = XMLElement::new("bpm");
    add_s32_element(&mut bpm_, "time", 0);
    add_s32_element(&mut bpm_, "delta_time", 0);
    add_s32_element(&mut bpm_, "bpm", bpm);
    bpm_info.add_child(bpm_).unwrap();
    info.add_child(bpm_info).unwrap();

    let mut measure_info = XMLElement::new("measure_info");

    let mut measure = XMLElement::new("measure");
    add_s32_element(&mut measure, "time", 0);
    add_s32_element(&mut measure, "delta_time", 0);
    add_s32_element(&mut measure, "num", 4);
    add_s32_element(&mut measure, "denomi", 4);
    measure_info.add_child(measure).unwrap();

    info.add_child(measure_info).unwrap();

    xml.add_child(info).unwrap();

    xml
}

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let input = std::fs::read_to_string("test.ssf").unwrap();
    let mut input_lines = input.lines();

    let mut bpm = 0;

    loop {
        let line_raw = input_lines.next().unwrap();
        if !line_raw.starts_with("#") {
            continue;
        }
        let line = line_raw.strip_prefix("#").unwrap();
        if let Some((command, argument)) = line.split_once(" ") {
            match command {
                "TITLE" => {
                    println!("Title: {}", argument);
                }
                "ARTIST" => {
                    println!("Artist: {}", argument);
                }
                "DESIGNER" => {
                    println!("Designer: {}", argument);
                }
                "DIFFICULTY" => {
                    println!(
                        "Difficulty: {}",
                        match argument {
                            "0" => "Easy",
                            "1" => "Normal",
                            "2" => "Hard",
                            _ => panic!(),
                        }
                    );
                }
                "PLAYLEVEL" => {
                    println!("Level: {}", argument);
                }
                "SONGID" => {
                    println!("Song ID: {}", argument);
                }
                "WAVE" => {
                    println!("Sound File: {}", argument);
                }
                "WAVEOFFSET" => {
                    println!("Sound Offset: {}", argument);
                }
                "JACKET" => {
                    println!("Cover Image: {}", argument);
                }
                "BPM01:" => {
                    println!("BPM: {}", argument);
                    bpm = argument.parse::<u32>().unwrap();
                }
                "00008:" => {
                    println!("Padding Bars: {}", argument);
                }
                _ => {}
            }
        } else {
            break;
        }
    }

    let mut measures = Vec::<Measure>::new();
    let mut current_measure = 0;

    loop {
        if let Some(line) = input_lines.next() {
            if line.contains("END") {
                break;
            }
            if !line.contains(":") {
                current_measure = line.parse::<usize>().unwrap();
                while measures.len() < current_measure + 1 {
                    measures.push(Measure::new());
                }
            } else {
                let (tick, notes) = line.split_once(":").unwrap();
                for note_string in notes.split(",") {
                    if note_string.len() == 0 {
                        continue;
                    }
                    measures[current_measure].ticks[tick.parse::<usize>().unwrap()]
                        .push(NoteEvent::from_string(note_string.to_string()));
                }
                //                 println!("Measure: {}, Tick: {}, Notes: {:?}", current_measure, tick, measures[current_measure].ticks[tick.parse::<usize>().unwrap()]);
            }
        } else {
            break;
        }
    }

    let mut builder = XMLBuilder::new()
        .version(XMLVersion::XML1_0)
        .encoding("UTF-8".into())
        .build();

    let mut data = xml_boilerplate(bpm * 100);

    let mut sequence_data = XMLElement::new("sequence_data");

    for (measure_num, measure) in measures.iter().enumerate() {
        for (tick_num, tick) in measure.ticks.iter().enumerate() {
            if tick.len() == 0 {
                continue;
            }
            println!(
                "Measure: {:?}, Tick: {:?}, Notes: {:?}",
                measure_num, tick_num, tick
            );
            println!(
                "ms: {:?}",
                measure_tick_to_ms(measure_num as u32, tick_num as u32, bpm)
            );
            for event in tick {
                match event {
                    NoteEvent::LeftStep { lane, width } | NoteEvent::RightStep { lane, width } => {
                        let mut step = XMLElement::new("step");
                        let time = measure_tick_to_ms(measure_num as u32, tick_num as u32, bpm);

                        add_s64_element(&mut step, "stime_ms", time.into());
                        add_s64_element(&mut step, "etime_ms", time.into());
                        add_s32_element(&mut step, "stime_dt", ms_to_dt(time, bpm).into());
                        add_s32_element(&mut step, "etime_dt", ms_to_dt(time, bpm).into());
                        add_s32_element(&mut step, "category", 0);
                        add_s32_element(&mut step, "pos_left", (*lane as u32 * 4096).into());
                        add_s32_element(
                            &mut step,
                            "pos_right",
                            ((lane + width) as u32 * 4096).into(),
                        );
                        add_s32_element(
                            &mut step,
                            "kind",
                            match event {
                                NoteEvent::LeftStep { .. } => 1,
                                NoteEvent::RightStep { .. } => 2,
                                _ => panic!(),
                            },
                        );
                        add_s32_element(&mut step, "var", 0);
                        add_s32_element(&mut step, "player_id", 0);

                        sequence_data.add_child(step).unwrap();
                    }
                    NoteEvent::Jump | NoteEvent::Down => {
                        let mut step = XMLElement::new("step");
                        let time = measure_tick_to_ms(measure_num as u32, tick_num as u32, bpm);

                        add_s64_element(&mut step, "stime_ms", time.into());
                        add_s64_element(&mut step, "etime_ms", time.into());
                        add_s32_element(&mut step, "stime_dt", ms_to_dt(time, bpm).into());
                        add_s32_element(&mut step, "etime_dt", ms_to_dt(time, bpm).into());
                        add_s32_element(&mut step, "category", 0);
                        add_s32_element(&mut step, "pos_left", 0);
                        add_s32_element(&mut step, "pos_right", 65536);
                        add_s32_element(
                            &mut step,
                            "kind",
                            match event {
                                NoteEvent::Down => 3,
                                NoteEvent::Jump => 4,
                                _ => panic!(),
                            },
                        );
                        add_s32_element(&mut step, "var", 0);
                        add_s32_element(&mut step, "player_id", 4);

                        sequence_data.add_child(step).unwrap();
                    }
                    NoteEvent::LeftHoldStart { id, lane, width }
                    | NoteEvent::RightHoldStart { id, lane, width } => {
                        let mut step = XMLElement::new("step");
                        let time = measure_tick_to_ms(measure_num as u32, tick_num as u32, bpm);
                        let mut end_time = time;

                        let mut waypoints = Vec::<LongPoint>::new();

                        // find end of hold in this measure and all future measures
                        'outer: for (end_tick_num, end_tick) in
                            measure.ticks.iter().enumerate().skip(tick_num)
                        {
                            for possible_end_event in end_tick {
                                match possible_end_event {
                                    NoteEvent::SlideEnd {
                                        id: end_id,
                                        lane: end_lane,
                                        width: end_width,
                                    } => {
                                        if end_id == id {
                                            waypoints.push(LongPoint::Normal {
                                                point_time: measure_tick_to_ms(
                                                    measure_num as u32,
                                                    end_tick_num as u32,
                                                    bpm,
                                                ),
                                                pos_left: *end_lane as u32 * 4096,
                                                pos_right: (*end_lane + end_width) as u32 * 4096,
                                            });
                                            end_time = measure_tick_to_ms(
                                                measure_num as u32,
                                                end_tick_num as u32,
                                                bpm,
                                            );
                                            break 'outer;
                                        }
                                    }
                                    NoteEvent::SlideWaypoint {
                                        id: point_id,
                                        lane,
                                        width,
                                    } => {
                                        if id == point_id {
                                            waypoints.push(LongPoint::Normal {
                                                point_time: measure_tick_to_ms(
                                                    measure_num as u32,
                                                    end_tick_num as u32,
                                                    bpm,
                                                ),
                                                pos_left: *lane as u32 * 4096,
                                                pos_right: (*lane + width) as u32 * 4096,
                                            });
                                        }
                                    }
                                    NoteEvent::SimpleSkidEnd {
                                        id: end_id,
                                        lane: end_lane,
                                        width: end_width,
                                    } => {
                                        if id == end_id {
                                            waypoints.push(LongPoint::SkidSimple {
                                                point_time: measure_tick_to_ms(
                                                    measure_num as u32,
                                                    end_tick_num as u32,
                                                    bpm,
                                                ),
                                                pos_left: *end_lane as u32 * 4096,
                                                pos_right: (*end_lane + end_width) as u32
                                                    * 4096,
                                            });
                                            end_time = measure_tick_to_ms(
                                                measure_num as u32,
                                                end_tick_num as u32,
                                                bpm,
                                            );
                                            break 'outer;
                                        }
                                    }
                                    NoteEvent::SimpleSkidWaypoint {
                                        id: point_id,
                                        lane: point_lane,
                                        width: point_width,
                                    } => {
                                        if id == point_id {
                                            waypoints.push(LongPoint::SkidSimple {
                                                point_time: measure_tick_to_ms(
                                                    measure_num as u32,
                                                    end_tick_num as u32,
                                                    bpm,
                                                ),
                                                pos_left: *point_lane as u32 * 4096,
                                                pos_right: (*point_lane + point_width) as u32
                                                    * 4096,
                                            });
                                        }
                                    }
                                    NoteEvent::ComplexSkidEnd {
                                        id: end_id,
                                        lane_start,
                                        width_start,
                                        lane_end,
                                        width_end,
                                    } => {
                                        if id == end_id {
                                            waypoints.push(LongPoint::SkidComplex {
                                                point_time: measure_tick_to_ms(
                                                    measure_num as u32,
                                                    end_tick_num as u32,
                                                    bpm,
                                                ),
                                                pos_left_start: *lane_start as u32 * 4096,
                                                pos_right_start: (*lane_start + width_start) as u32
                                                    * 4096,
                                                pos_left_end: *lane_end as u32 * 4096,
                                                pos_right_end: (*lane_end + width_end) as u32
                                                    * 4096,
                                            });
                                            end_time = measure_tick_to_ms(
                                                measure_num as u32,
                                                end_tick_num as u32,
                                                bpm,
                                            );
                                            break 'outer;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if end_time == time {
                            'outer: for (end_measure_num, end_measure) in
                                measures.iter().enumerate().skip(measure_num + 1)
                            {
                                for (end_tick_num, end_tick) in end_measure.ticks.iter().enumerate()
                                {
                                    for possible_end_event in end_tick {
                                        match possible_end_event {
                                            NoteEvent::SlideEnd {
                                                id: end_id,
                                                lane: end_lane,
                                                width: end_width,
                                            } => {
                                                if end_id == id {
                                                    waypoints.push(LongPoint::Normal {
                                                        point_time: measure_tick_to_ms(
                                                            end_measure_num as u32,
                                                            end_tick_num as u32,
                                                            bpm,
                                                        ),
                                                        pos_left: *end_lane as u32 * 4096,
                                                        pos_right: (*end_lane + end_width) as u32
                                                            * 4096,
                                                    });
                                                    end_time = measure_tick_to_ms(
                                                        end_measure_num as u32,
                                                        end_tick_num as u32,
                                                        bpm,
                                                    );
                                                    break 'outer;
                                                }
                                            }
                                            NoteEvent::SlideWaypoint {
                                                id: point_id,
                                                lane,
                                                width,
                                            } => {
                                                if id == point_id {
                                                    waypoints.push(LongPoint::Normal {
                                                        point_time: measure_tick_to_ms(
                                                            end_measure_num as u32,
                                                            end_tick_num as u32,
                                                            bpm,
                                                        ),
                                                        pos_left: *lane as u32 * 4096,
                                                        pos_right: (*lane + width) as u32 * 4096,
                                                    });
                                                }
                                            }
                                            NoteEvent::SimpleSkidEnd {
                                                id: end_id,
                                                lane: end_lane,
                                                width: end_width,
                                            } => {
                                                if id == end_id {
                                                    waypoints.push(LongPoint::SkidSimple {
                                                        point_time: measure_tick_to_ms(
                                                            end_measure_num as u32,
                                                            end_tick_num as u32,
                                                            bpm,
                                                        ),
                                                        pos_left: *end_lane as u32 * 4096,
                                                        pos_right: (*end_lane + end_width)
                                                            as u32
                                                            * 4096,
                                                    });
                                                    end_time = measure_tick_to_ms(
                                                        end_measure_num as u32,
                                                        end_tick_num as u32,
                                                        bpm,
                                                    );
                                                    break 'outer;
                                                }
                                            }
                                            NoteEvent::SimpleSkidWaypoint {
                                                id: point_id,
                                                lane: point_lane,
                                                width: point_width,
                                            } => {
                                                if id == point_id {
                                                    waypoints.push(LongPoint::SkidSimple {
                                                        point_time: measure_tick_to_ms(
                                                            end_measure_num as u32,
                                                            end_tick_num as u32,
                                                            bpm,
                                                        ),
                                                        pos_left: *point_lane as u32 * 4096,
                                                        pos_right: (*point_lane + point_width)
                                                            as u32
                                                            * 4096,
                                                    });
                                                }
                                            }
                                            NoteEvent::ComplexSkidEnd {
                                                id: end_id,
                                                lane_start,
                                                width_start,
                                                lane_end,
                                                width_end,
                                            } => {
                                                if id == end_id {
                                                    waypoints.push(LongPoint::SkidComplex {
                                                        point_time: measure_tick_to_ms(
                                                            measure_num as u32,
                                                            end_tick_num as u32,
                                                            bpm,
                                                        ),
                                                        pos_left_start: *lane_start as u32 * 4096,
                                                        pos_right_start: (*lane_start + width_start)
                                                            as u32
                                                            * 4096,
                                                        pos_left_end: *lane_end as u32 * 4096,
                                                        pos_right_end: (*lane_end + width_end)
                                                            as u32
                                                            * 4096,
                                                    });
                                                    end_time = measure_tick_to_ms(
                                                        measure_num as u32,
                                                        end_tick_num as u32,
                                                        bpm,
                                                    );
                                                    break 'outer;
                                                }
                                            }

                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }

                        add_s64_element(&mut step, "stime_ms", time.into());
                        add_s64_element(&mut step, "etime_ms", end_time.into());
                        add_s32_element(&mut step, "stime_dt", ms_to_dt(time, bpm).into());
                        add_s32_element(&mut step, "etime_dt", ms_to_dt(end_time, bpm).into());
                        add_s32_element(&mut step, "category", 1);
                        add_s32_element(&mut step, "pos_left", (*lane as u32 * 4096).into());
                        add_s32_element(
                            &mut step,
                            "pos_right",
                            ((lane + width) as u32 * 4096).into(),
                        );
                        add_s32_element(
                            &mut step,
                            "kind",
                            match event {
                                NoteEvent::LeftHoldStart { .. } => 1,
                                NoteEvent::RightHoldStart { .. } => 2,
                                _ => panic!(),
                            },
                        );
                        add_s32_element(&mut step, "var", 0);
                        add_s32_element(&mut step, "player_id", 0);

                        let mut long_point = XMLElement::new("long_point");

                        let mut last_left = *lane as u32 * 4096;
                        let mut last_right = (*lane + width) as u32 * 4096;

                        for waypoint in waypoints {
                            let mut point = XMLElement::new("point");
                            match waypoint {
                                LongPoint::Normal {
                                    point_time,
                                    pos_left,
                                    pos_right,
                                } => {
                                    add_s64_element(&mut point, "point_time", point_time.into());
                                    add_s32_element(&mut point, "pos_left", pos_left);
                                    add_s32_element(&mut point, "pos_right", pos_right);
                                }
                                LongPoint::SkidComplex {
                                    point_time,
                                    pos_left_start,
                                    pos_right_start,
                                    pos_left_end,
                                    pos_right_end,
                                } => {
                                    add_s64_element(&mut point, "point_time", point_time.into());
                                    add_s32_element(&mut point, "pos_left", pos_left_start);
                                    add_s32_element(&mut point, "pos_right", pos_right_start);
                                    add_s32_element(&mut point, "pos_lend", pos_left_end);
                                    add_s32_element(&mut point, "pos_rend", pos_right_end);
                                    last_left = pos_left_end;
                                    last_right = pos_right_end;
                                }
                                LongPoint::SkidSimple {
                                    point_time,
                                    pos_left,
                                    pos_right,
                                } => {
                                    add_s64_element(&mut point, "point_time", point_time.into());
                                    add_s32_element(&mut point, "pos_left", last_left);
                                    add_s32_element(&mut point, "pos_right", last_right);
                                    if pos_right > last_right {
                                        add_s32_element(&mut point, "pos_lend", (pos_left + pos_right) / 2);
                                        add_s32_element(&mut point, "pos_rend", pos_right);
                                    } else {
                                        add_s32_element(&mut point, "pos_lend", pos_left);
                                        add_s32_element(&mut point, "pos_rend", (pos_left + pos_right) / 2);
                                    }
                                    last_left = pos_left;
                                    last_right = pos_right;
                                }
                                _ => {}
                            }
                            long_point.add_child(point).unwrap();
                        }
                        step.add_child(long_point).unwrap();

                        sequence_data.add_child(step).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }

    data.add_child(sequence_data).unwrap();

    builder.set_root_element(data);
    let mut writer = Vec::<u8>::new();
    builder.generate(&mut writer).unwrap();
    std::fs::write("output.xml", writer).unwrap();
}
