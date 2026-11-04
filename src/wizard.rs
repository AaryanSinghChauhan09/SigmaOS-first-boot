#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep { Language, Timezone, KeyboardLayout, UserAccount, Theme, Finish }
#[derive(Debug, Clone)]
pub struct FirstBootWizard {
    pub current_step: WizardStep,
    pub language: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub username: String,
    pub theme: String,
    completed: bool,
}
impl FirstBootWizard {
    pub fn new() -> Self {
        Self {
            current_step: WizardStep::Language,
            language: "en_US".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
            username: String::new(),
            theme: "sigma-default".to_string(),
            completed: false,
        }
    }
    pub fn next_step(&mut self) {
        self.current_step = match self.current_step {
            WizardStep::Language => WizardStep::Timezone,
            WizardStep::Timezone => WizardStep::KeyboardLayout,
            WizardStep::KeyboardLayout => WizardStep::UserAccount,
            WizardStep::UserAccount => WizardStep::Theme,
            WizardStep::Theme => WizardStep::Finish,
            WizardStep::Finish => WizardStep::Finish,
        };
    }
    pub fn complete(&mut self) -> Result<(), String> {
        if self.username.is_empty() { return Err("username required".to_string()); }
        self.completed = true;
        Ok(())
    }
    pub fn is_complete(&self) -> bool { self.completed }
    pub fn step_index(&self) -> usize {
        match self.current_step {
            WizardStep::Language => 0,
            WizardStep::Timezone => 1,
            WizardStep::KeyboardLayout => 2,
            WizardStep::UserAccount => 3,
            WizardStep::Theme => 4,
            WizardStep::Finish => 5,
        }
    }
}
impl Default for FirstBootWizard { fn default() -> Self { Self::new() } }
