pub enum Action {
    None,
    Initialize,
    Quit,
    LoadBusNames,
    LoadPaths(String),
    SelectLastBusName,
    SelectNextBusName,
}
