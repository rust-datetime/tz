extern crate byteorder;
use std::rc::Rc;

pub mod internals;

/// The 'type' of time that the change was announced in.
#[derive(Debug, PartialEq, Eq, Clone)]
enum TransitionType {

    /// Standard Time ("non-summer" time)
    Standard,

    /// Wall clock time
    Wall,

    // Co-ordinated Universal Time
    UTC,
}

/// A time change specification.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transition {

    /// Unix timestamp when the clocks change.
    timestamp: u32,

    /// The new description of the local time.
    local_time_type: Rc<LocalTimeType>,
}

/// A leap second specification.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LeapSecond {

    /// Unix timestamp at which a leap second occurs.
    timestamp: u32,

    /// Number of leap seconds to be added.
    leap_second_count: u32,
}

/// A description of the local time in a particular timezone, during the
/// period in which the clocks do not change.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocalTimeType {

    /// The time zone abbreviation - such as "GMT" or "UTC".
    name: String,

    /// Number of seconds to be added to Universal Time.
    offset: u32,

    /// Whether to set DST.
    is_dst: bool,

    /// The current 'type' of time.
    transition_type: TransitionType,
}

/// Convert the internal time zone data into a list of transitions.
pub fn cook(tz: internals::TZData) -> Option<Vec<Transition>> {
    let mut transitions = Vec::new();
    let mut local_time_types = Vec::new();

    // First, build up a list of local time types...
    for i in 0 .. tz.header.num_time_types as usize {
        let ref data = tz.time_info[i];

        // Isolate the relevant bytes by the index of the start of the
        // string and the next available null char
        let name_bytes = tz.strings.iter()
                                   .cloned()
                                   .skip(data.name_offset as usize)
                                   .take_while(|&c| c != 0)
                                   .collect();

        let info = LocalTimeType {
            name:             String::from_utf8(name_bytes).unwrap(),
            offset:           data.offset,
            is_dst:           data.is_dst != 0,
            transition_type:  flags_to_transition_type(tz.standard_flags[i] != 0,
                                                       tz.utc_flags[i]      != 0),
        };

        local_time_types.push(Rc::new(info));
    }

    // ...then, link each transition with the time type it refers to.
    for i in 0 .. tz.header.num_transition_times as usize {
        let ref t = tz.transitions[i];

        let transition = Transition {
            timestamp:        t.timestamp,
            local_time_type:  local_time_types[t.local_time_type_index as usize].clone(),
        };

        transitions.push(transition);
    }

    Some(transitions)
}

fn flags_to_transition_type(standard: bool, utc: bool) -> TransitionType {
    match (standard, utc) {
        (_,     true)   => TransitionType::UTC,
        (true,  _)      => TransitionType::Standard,
        (false, false)  => TransitionType::Wall,
    }
}