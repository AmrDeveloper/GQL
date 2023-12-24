use crate::tokenizer::Location;

/// In Memory representation for the Diagnostic element
pub struct Diagnostic {
    label: String,
    message: String,
    location: Option<(usize, usize)>,
    notes: Vec<String>,
    helps: Vec<String>,
    docs: Option<String>,
}

impl Diagnostic {
    /// Create new instance of Diagnostic with required label and message
    #[must_use]
    pub fn new(label: &str, message: &str) -> Self {
        Diagnostic {
            label: label.to_owned(),
            message: message.to_owned(),
            location: None,
            notes: vec![],
            helps: vec![],
            docs: None,
        }
    }

    /// Create new instance of Diagnostic with label `Error`
    #[must_use]
    pub fn error(message: &str) -> Self {
        Diagnostic {
            label: "Error".to_owned(),
            message: message.to_owned(),
            location: None,
            notes: vec![],
            helps: vec![],
            docs: None,
        }
    }

    /// Create new instance of Diagnostic with label `Exception`
    #[must_use]
    pub fn exception(message: &str) -> Self {
        Diagnostic {
            label: "Exception".to_owned(),
            message: message.to_owned(),
            location: None,
            notes: vec![],
            helps: vec![],
            docs: None,
        }
    }

    /// Set location start and end from Location type
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some((location.start, location.end));
        self
    }

    /// Set location start and end
    pub fn with_location_span(mut self, start: usize, end: usize) -> Self {
        self.location = Some((start, end));
        self
    }

    /// Add new note to the current list
    pub fn add_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_owned());
        self
    }

    /// Add new help to the current list
    pub fn add_help(mut self, help: &str) -> Self {
        self.helps.push(help.to_owned());
        self
    }

    /// Set Docs url
    pub fn with_docs(mut self, docs: &str) -> Self {
        self.docs = Some(docs.to_owned());
        self
    }

    /// Return the Diagnostic label
    pub fn label(&self) -> &String {
        &self.label
    }

    /// Return the Diagnostic message
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Return the diagnostic location span (column start and end)
    pub fn location(&self) -> Option<(usize, usize)> {
        self.location
    }

    /// Return the list of notes messages
    pub fn notes(&self) -> &Vec<String> {
        &self.notes
    }

    /// Return the list of helps messages
    pub fn helps(&self) -> &Vec<String> {
        &self.helps
    }

    /// Return the docs url if exists
    pub fn docs(&self) -> &Option<String> {
        &self.docs
    }

    /// Get the Diagnostic as Box::<Diagnostic>
    pub fn as_boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
