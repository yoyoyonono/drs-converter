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

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let input = std::fs::read_to_string("test.ssf").unwrap();
    let mut input_lines = input.lines();
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

    for (measure_num, measure) in measures.iter().enumerate() {
        for (tick_num, tick) in measure.ticks.iter().enumerate() {
            if tick.len() == 0 {
                continue;
            }
            println!("Measure: {:?}, Tick: {:?}, Notes: {:?}", measure_num, tick_num, tick);
        }
    }
}
