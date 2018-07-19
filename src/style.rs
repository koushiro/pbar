#[derive(Clone)]
pub enum LayoutElem {
    Title,
    Current,
    Total,
    Percent,
    Bar,
    TimeLeft,
    TimeElapsed,
    TimeTotal,
    Speed,
    Separator(String),
}

pub enum ElemColor {

}


#[derive(Clone)]
pub struct ProgressBarStyle {
    pub bar_symbols: Vec<char>,
    layout: Vec<LayoutElem>,
}

impl ProgressBarStyle {
    /// Return the default progress bar style.
    pub fn default() -> ProgressBarStyle {
        ProgressBarStyle {
            bar_symbols: "[#>-]".chars().collect(),
            layout: vec![
                LayoutElem::Title,
                LayoutElem::Speed,
                LayoutElem::TimeLeft,
                LayoutElem::Percent,
                LayoutElem::Bar,
            ],
        }
    }

    /// Return the progress bar style without any content.
    fn customizable() -> ProgressBarStyle {
        ProgressBarStyle {
            bar_symbols: vec![],
            layout: vec![],
        }
    }

    /// Set the bar symbols `(begin, fill, current, empty, end)`.
    pub fn set_bar_symbols(&mut self, s: &str) -> &ProgressBarStyle {
        self.bar_symbols = s.chars().collect();
        self
    }

    fn title(&mut self, s: &str) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::Title);
        self
    }

    fn current(&mut self, value: u64) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::Current);
        self
    }

    fn total(&mut self, value: u64) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::Total);
        self
    }

    fn percent(&mut self) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::Percent);
        self
    }

    fn bar(&mut self) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::Bar);
        self
    }

    fn time_left(&mut self) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::TimeLeft);
        self
    }

    fn time_elapsed(&mut self) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::TimeElapsed);
        self
    }

    fn time_total(&mut self) -> &ProgressBarStyle {
        self.layout.push(LayoutElem::TimeTotal);
        self
    }

    fn separate(&mut self, s: &str) ->&ProgressBarStyle {
        self.layout.push(LayoutElem::Separator(s.to_string()));
        self
    }
}