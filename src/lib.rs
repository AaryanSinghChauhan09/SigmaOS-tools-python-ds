/// Python notebook cell model and kernel execution bridge for SigmaOS Jupyter-compat.

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Code,
    Markdown,
    Raw,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionState {
    Idle,
    Busy,
    Error,
}

#[derive(Debug, Clone)]
pub struct NotebookCell {
    pub idx: usize,
    pub ctype: CellType,
    pub source: String,
    pub output: Vec<String>,
    pub exec_count: Option<u32>,
    pub state: ExecutionState,
}

impl NotebookCell {
    pub fn new(idx: usize, ctype: CellType, source: &str) -> Self {
        Self {
            idx,
            ctype,
            source: source.into(),
            output: Vec::new(),
            exec_count: None,
            state: ExecutionState::Idle,
        }
    }

    pub fn execute(&mut self, output: Vec<String>, count: u32) {
        self.output = output;
        self.exec_count = Some(count);
        self.state = ExecutionState::Idle;
    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }
    pub fn is_code(&self) -> bool {
        self.ctype == CellType::Code
    }
}

#[derive(Debug, Default)]
pub struct Notebook {
    pub cells: Vec<NotebookCell>,
    pub kernel: String,
}

impl Notebook {
    pub fn add_cell(&mut self, c: NotebookCell) {
        self.cells.push(c);
    }
    pub fn code_cell_count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_code()).count()
    }
    pub fn executed_count(&self) -> usize {
        self.cells.iter().filter(|c| c.exec_count.is_some()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_cell_stores_output_aaa() {
        let mut c = NotebookCell::new(0, CellType::Code, "print('hello')");
        c.execute(vec!["hello".into()], 1);
        assert!(c.has_output());
        assert_eq!(c.exec_count, Some(1));
    }

    #[test]
    fn test_code_cell_type_detection_aaa() {
        let c = NotebookCell::new(0, CellType::Code, "x = 1");
        assert!(c.is_code());
    }

    #[test]
    fn test_notebook_code_cell_count_aaa() {
        let mut nb = Notebook {
            kernel: "python3".into(),
            cells: Vec::new(),
        };
        nb.add_cell(NotebookCell::new(0, CellType::Code, ""));
        nb.add_cell(NotebookCell::new(1, CellType::Markdown, ""));
        assert_eq!(nb.code_cell_count(), 1);
    }

    #[test]
    fn test_notebook_executed_count_aaa() {
        let mut nb = Notebook {
            kernel: "python3".into(),
            cells: Vec::new(),
        };
        let mut c = NotebookCell::new(0, CellType::Code, "1+1");
        c.execute(vec!["2".into()], 1);
        nb.add_cell(c);
        nb.add_cell(NotebookCell::new(1, CellType::Code, "unrun"));
        assert_eq!(nb.executed_count(), 1);
    }
}
