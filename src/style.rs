use format::*;

#[derive(Clone)]
pub enum Component {
    Counter(String, UnitFormat), // layout - Current Str("/") Total
    Speed(UnitFormat),
    Percent,
    Bar(Vec<char>, usize),
    TimeLeft(TimeFormat),
    TimeElapsed(TimeFormat),
    TimeTotal(TimeFormat),
    Str(String),
}

#[derive(Clone)]
pub struct ProgressBarStyle {
    pub layout: Vec<Component>,
}

impl ProgressBarStyle {
    /// Return a default progress bar style,
    /// including 'Counter','Percent','Bar','TimeLeft' components.
    pub fn default() -> ProgressBarStyle {
        ProgressBarStyle {
            layout: vec![
                Component::Counter("/".to_string(), UnitFormat::Default),
                Component::Percent,
                Component::Bar("[#>-]".chars().collect(), 30),
                Component::TimeLeft(TimeFormat::Fmt1),
            ],
        }
    }

    /// Return a customizable progress bar style without any content.
    pub fn customizable() -> ProgressBarStyle {
        ProgressBarStyle { layout: vec![] }
    }

    /// Add 'counter' component to the style,
    /// default delimiter is '/'; default format is pure number.
    pub fn counter(&mut self, delimiter: Option<String>, fmt: Option<UnitFormat>) -> &mut Self {
        self.layout.push(Component::Counter(
            delimiter.unwrap_or_else(|| "/".to_string()),
            fmt.unwrap_or(UnitFormat::Default),
        ));
        self
    }

    /// Add 'percent' component to the style.
    pub fn percent(&mut self) -> &mut Self {
        self.layout.push(Component::Percent);
        self
    }

    /// Add 'bar' component to the style,
    /// default bar width is 30.
    pub fn bar(&mut self, s: &str, width: Option<usize>) -> &mut Self {
        self.layout
            .push(Component::Bar(s.chars().collect(), width.unwrap_or(30)));
        self
    }

    /// Add 'time_left' component to the style,
    /// default format is like MM:SS | HH:MM:SS | XX..Xd:HH:MM::SS.
    pub fn time_left(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout
            .push(Component::TimeLeft(fmt.unwrap_or(TimeFormat::Fmt1)));
        self
    }

    /// Add 'time_elapsed' component to the style,
    /// default format is like MM:SS | HH:MM:SS | XX..Xd:HH:MM::SS.
    pub fn time_elapsed(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout
            .push(Component::TimeElapsed(fmt.unwrap_or(TimeFormat::Fmt1)));
        self
    }

    /// Add 'time_total' component to the style,
    /// default format is like MM:SS | HH:MM:SS | XX..Xd:HH:MM::SS.
    pub fn time_total(&mut self, fmt: Option<TimeFormat>) -> &mut Self {
        self.layout
            .push(Component::TimeTotal(fmt.unwrap_or(TimeFormat::Fmt1)));
        self
    }

    /// Add 'speed' component to the style,
    /// default format is pure number.
    pub fn speed(&mut self, fmt: Option<UnitFormat>) -> &mut Self {
        self.layout
            .push(Component::Speed(fmt.unwrap_or(UnitFormat::Default)));
        self
    }

    /// Add 'str' component to the style, like delimiter.
    pub fn str(&mut self, s: &str) -> &mut Self {
        self.layout.push(Component::Str(s.to_string()));
        self
    }
}
