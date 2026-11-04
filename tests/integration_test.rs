use sigmaos_first_boot::{Config, FirstBootComponent, FirstBootWizard, WizardStep, Component};

#[test]
fn test_wizard_initial_step_aaa() {
    let wizard = FirstBootWizard::new();
    assert_eq!(wizard.current_step, WizardStep::Language);
    assert_eq!(wizard.step_index(), 0);
}

#[test]
fn test_wizard_next_step_aaa() {
    let mut wizard = FirstBootWizard::new();
    wizard.next_step();
    assert_eq!(wizard.current_step, WizardStep::Timezone);
    wizard.next_step();
    assert_eq!(wizard.current_step, WizardStep::KeyboardLayout);
}

#[test]
fn test_wizard_complete_requires_username_aaa() {
    let mut wizard = FirstBootWizard::new();
    let result = wizard.complete();
    assert!(result.is_err());
    assert!(!wizard.is_complete());
}

#[test]
fn test_wizard_complete_with_username_aaa() {
    let mut wizard = FirstBootWizard::new();
    wizard.username = "alice".to_string();
    let result = wizard.complete();
    assert!(result.is_ok());
    assert!(wizard.is_complete());
}

#[test]
fn test_wizard_full_flow_aaa() {
    let mut wizard = FirstBootWizard::new();
    wizard.language = "de_DE".to_string();
    wizard.timezone = "Europe/Berlin".to_string();
    wizard.username = "max".to_string();
    for _ in 0..5 { wizard.next_step(); }
    assert_eq!(wizard.current_step, WizardStep::Finish);
    assert!(wizard.complete().is_ok());
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = FirstBootComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = FirstBootComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-first-boot");
}
