use std::borrow::Cow;

pub enum StepResult {
    Success,
    Failure,
    Ignored,
    Skipped(String),
}

impl StepResult {
    pub fn failed(&self) -> bool {
        matches!(self, StepResult::Failure)
    }
}

type CowString<'a> = Cow<'a, str>;
type ReportData<'a> = Vec<(CowString<'a>, StepResult)>;
pub struct Report<'a> {
    data: ReportData<'a>,
}

impl<'a> Report<'a> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push_result<M>(&mut self, result: Option<(M, StepResult)>)
    where
        M: Into<CowString<'a>>,
    {
        if let Some((key, success)) = result {
            let key = key.into();

            debug_assert!(!self.data.iter().any(|(k, _)| k == &key), "{} already reported", key);
            self.data.push((key, success));
        }
    }

    pub fn data(&self) -> &ReportData<'a> {
        &self.data
    }
}
