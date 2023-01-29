
pub fn time_selection_values_hours(h: u8) -> [char; 2] {
    match h {
        0 => ['0', '0'],
        1 => ['0', '1'],
        2 => ['0', '2'],
        3 => ['0', '3'],
        4 => ['0', '4'],
        5 => ['0', '5'],
        6 => ['0', '6'],
        7 => ['0', '7'],
        8 => ['0', '8'],
        9 => ['0', '9'],
        10 => ['1', '0'],
        11 => ['1', '1'],
        12 => ['1', '2'],
        13 => ['1', '3'],
        14 => ['1', '4'],
        15 => ['1', '5'],
        16 => ['1', '6'],
        17 => ['1', '7'],
        18 => ['1', '8'],
        19 => ['1', '9'],
        20 => ['2', '0'],
        21 => ['2', '1'],
        22 => ['2', '2'],
        23 => ['2', '3'],
        _ => ['^', '^']
    }
}

pub fn time_selection_values_minutes(m: u8) -> [char; 2] {
    match m {
        0 => ['0', '0'],
        1 => ['1', '5'],
        2 => ['3', '0'],
        3 => ['4', '5'],
        _ => ['^', '^']
    }
}

pub struct TimeSeriesSelections {
    hours_selection_index: u8,
    minutes_selection_index: u8,
    hours_index_max_val: u8,
    minutes_index_max_val: u8
}

impl TimeSeriesSelections {
    pub fn new() -> TimeSeriesSelections {
        TimeSeriesSelections {
            hours_selection_index: 0,
            minutes_selection_index: 0,
            hours_index_max_val: 23,
            minutes_index_max_val: 3
        }
    }
    pub fn increment_hours_selection_index(&mut self) {
        if self.hours_selection_index < self.hours_index_max_val {
            self.hours_selection_index += 1;
        } else {
            self.hours_selection_index = 0;
        }
    }
    pub fn increment_minutes_selection_index(&mut self) {
        if self.minutes_selection_index < self.minutes_index_max_val {
            self.minutes_selection_index += 1;
        } else {
            self.minutes_selection_index = 0;
        }
    }
    pub fn get_time_selection(&mut self) -> [char; 5] {
        let mut time_selection = [' ', ' ', ':', ' ', ' '];
        let hours = time_selection_values_hours(
            self.hours_selection_index
        );
        let minutes = time_selection_values_minutes(
            self.minutes_selection_index
        );
        time_selection[0] = hours[0];
        time_selection[1] = hours[1];
        time_selection[3] = minutes[0];
        time_selection[4] = minutes[1];
        time_selection
    }
}
