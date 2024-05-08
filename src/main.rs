use xml_builder::{XMLBuilder, XMLElement, XMLVersion, XML};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoteEvent {
    LeftStep{lane: u8, width: u8},
    RightStep{lane: u8, width: u8},
    Jump,
    Down,
    LeftHoldStart{id: u8, lane: u8, width: u8},
    RightHoldStart{id: u8, lane: u8, width: u8},
    SlideWaypoint{id: u8, lane: u8, width: u8},
    SlideEnd{id: u8, lane: u8, width: u8},
    SimpleSkidWaypoint{id: u8, lane: u8, width: u8},
    ComplexSkidWaypoint{id: u8, lane_start: u8, width_start: u8, lane_end: u8, width_end: u8},
    SimpleSkidEnd{id: u8, lane: u8, width: u8},
    ComplexSkidEnd{id: u8, lane_start: u8, width_start: u8, lane_end: u8, width_end: u8},
}

impl NoteEvent {
    fn from_string(input_string: String) -> NoteEvent {
        let chars: Vec::<char> = input_string.chars().collect();
        match chars[0] {
            '0' => {NoteEvent::LeftStep { 
                lane: u8::from_str_radix(&chars[1].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[2].to_string(), 16).unwrap() + 1
            }},
            '1' => {NoteEvent::RightStep { 
                lane: u8::from_str_radix(&chars[1].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[2].to_string(), 16).unwrap() + 1
            }},
            '2' => {NoteEvent::Jump},
            '3' => {NoteEvent::Down},
            '4' => {NoteEvent::LeftHoldStart { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            '5' => {NoteEvent::RightHoldStart { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            '6' => {NoteEvent::SlideWaypoint { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            '7' => {NoteEvent::SlideEnd { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            '8' => {NoteEvent::SimpleSkidWaypoint { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            '9' => {NoteEvent::ComplexSkidWaypoint { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane_start: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width_start: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
                lane_end: u8::from_str_radix(&chars[4].to_string(), 16).unwrap(), 
                width_end: u8::from_str_radix(&chars[5].to_string(), 16).unwrap() + 1
            }},
            'A' => {NoteEvent::SimpleSkidEnd { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1
            }},
            'B' => {NoteEvent::ComplexSkidEnd { 
                id: u8::from_str_radix(&chars[1].to_string(), 36).unwrap(), 
                lane_start: u8::from_str_radix(&chars[2].to_string(), 16).unwrap(), 
                width_start: u8::from_str_radix(&chars[3].to_string(), 16).unwrap() + 1,
                lane_end: u8::from_str_radix(&chars[4].to_string(), 16).unwrap(), 
                width_end: u8::from_str_radix(&chars[5].to_string(), 16).unwrap() + 1
            }},
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Measure {
    ticks: Vec::<Vec::<NoteEvent>>
}

impl Measure {
    fn new() -> Measure {
        Measure {
            ticks : vec![Vec::<NoteEvent>::new(); 192]
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

fn xml_boilerplate(bpm: u32) -> XMLElement {
    let mut xml = XMLElement::new("data");
    
    let mut seq_version = XMLElement::new("seq_version");
    seq_version.add_attribute("__type", "s32");
    seq_version.add_text("8".into()).unwrap();
    xml.add_child(seq_version).unwrap();

    let mut info = XMLElement::new("info");

    let mut tick = XMLElement::new("tick");
    tick.add_attribute("__type", "s32");
    tick.add_text("480".into()).unwrap();
    info.add_child(tick).unwrap();


    let mut bpm_info = XMLElement::new("bpm_info");
    let mut bpm_ = XMLElement::new("bpm");
    let mut time = XMLElement::new("time");
    time.add_attribute("__type", "s32");
    time.add_text("0".into()).unwrap();
    bpm_.add_child(time).unwrap();

    let mut delta_time = XMLElement::new("delta_time");
    delta_time.add_attribute("__type", "s32");
    delta_time.add_text("0".into()).unwrap();
    bpm_.add_child(delta_time).unwrap();

    let mut bpm_value = XMLElement::new("bpm");
    bpm_value.add_attribute("__type", "s32");
    bpm_value.add_text(bpm.to_string()).unwrap();
    bpm_.add_child(bpm_value).unwrap();
    bpm_info.add_child(bpm_).unwrap();
    info.add_child(bpm_info).unwrap();

   
    let mut measure_info = XMLElement::new("measure_info");

    let mut measure = XMLElement::new("measure");
    let mut measure_time = XMLElement::new("time");
    measure_time.add_attribute("__type", "s32");
    measure_time.add_text("0".into()).unwrap();
    measure.add_child(measure_time).unwrap();

    let mut measure_delta_time = XMLElement::new("delta_time");
    measure_delta_time.add_attribute("__type", "s32");
    measure_delta_time.add_text("0".into()).unwrap();
    measure.add_child(measure_delta_time).unwrap();

    let mut measure_num = XMLElement::new("num");
    measure_num.add_attribute("__type", "s32");
    measure_num.add_text("4".into()).unwrap();
    measure.add_child(measure_num).unwrap();

    let mut measure_denomi = XMLElement::new("denomi");
    measure_denomi.add_attribute("__type", "s32");
    measure_denomi.add_text("4".into()).unwrap();
    measure.add_child(measure_denomi).unwrap();
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
                    println!("Difficulty: {}", 
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
                _ => {

                }
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
                if measures.len() < current_measure + 1 {
                    measures.push(Measure::new());
                }
            } else {
                let (tick, notes) = line.split_once(":").unwrap();
                for note_string in notes.split(",") {
                    if note_string.len() == 0 {
                        continue;
                    }
                    measures[current_measure].ticks[tick.parse::<usize>().unwrap()].push(NoteEvent::from_string(note_string.to_string()));
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
            println!("Measure: {:?}, Tick: {:?}, Notes: {:?}", measure_num, tick_num, tick);
            println!("ms: {:?}", measure_tick_to_ms(measure_num as u32, tick_num as u32, bpm));
        }
    }

    builder.set_root_element(data);
    let mut writer = Vec::<u8>::new();
    builder.generate(&mut writer).unwrap();
    std::fs::write("output.xml", writer).unwrap();
}
