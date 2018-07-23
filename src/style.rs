use format::*;

#[derive(Clone)]
pub enum Component {
    Counter(String, UnitFormat),    // Current Separator Total
    Speed(UnitFormat),
    Percent,
    Bar(Vec<char>, usize),
    TimeLeft(TimeFormat),
    TimeElapsed(TimeFormat),
    TimeTotal(TimeFormat),
    Delimiter(String),
}

#[derive(Clone)]
pub struct ProgressBarStyle {
    pub layout: Vec<Component>,
}

impl ProgressBarStyle {
    /// Return the default progress bar style.
    pub fn default() -> ProgressBarStyle {
        ProgressBarStyle {
            layout: vec![
                Component::TimeLeft(TimeFormat::TimeFmt3),
                Component::Counter("/".to_string(), UnitFormat::Default),
                Component::Percent,
                Component::Bar("[#>-]".chars().collect(), 30),
            ],
        }
    }

    /// Return the progress bar style without any content.
    pub fn customizable() -> ProgressBarStyle {
        ProgressBarStyle {
            layout: vec![],
        }
    }

    pub fn counter(&mut self, delimiter: Option<String>, fmt: Option<UnitFormat>)
        -> &mut Self
    {
        self.layout.push(
            Component::Counter(
                delimiter.unwrap_or("/".to_string()),
                fmt.unwrap_or(UnitFormat::Default),
            )
        );
        self
    }

    pub fn percent(&mut self) -> &mut Self {
        self.layout.push(
            Component::Percent
        );
        self
    }

    pub fn bar(&mut self, s: &str,  width: Option<usize>) -> &mut Self {
        self.layout.push(
            Component::Bar(
                s.chars().collect(),
                width.unwrap_or(30),
            )
        );
        self
    }

    pub fn time_left(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout.push(
            Component::TimeLeft(
                fmt.unwrap_or(TimeFormat::TimeFmt3),
            )
        );
        self
    }

    pub fn time_elapsed(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout.push(
            Component::TimeElapsed(
                fmt.unwrap_or(TimeFormat::TimeFmt3),
            )
        );
        self
    }

    pub fn time_total(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout.push(
            Component::TimeTotal(
                fmt.unwrap_or(TimeFormat::TimeFmt3),
            )
        );
        self
    }

    pub fn speed(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout.push(
            Component::TimeTotal(
                fmt.unwrap_or(TimeFormat::TimeFmt3),
            )
        );
        self
    }

    pub fn delimiter(&mut self, s: &str) -> &mut Self {
        self.layout.push(
            Component::Delimiter(
                s.to_string(),
            )
        );
        self
    }
}