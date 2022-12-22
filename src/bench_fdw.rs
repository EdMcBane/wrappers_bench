use std::collections::HashMap;
use supabase_wrappers::prelude::*;

// A simple bench FDW
#[wrappers_fdw(version = "0.1.0")]
pub(crate) struct BenchFdw {
    // target column name list
    cnt: usize,
    tgt_cols: Vec<String>,
}

impl ForeignDataWrapper for BenchFdw {
    fn new(_options: &HashMap<String, String>) -> Self {
        Self {
            tgt_cols: Vec::new(),
            cnt: 0,
        }
    }

    fn begin_scan(
        &mut self,
        _quals: &[Qual],
        columns: &[String],
        _sorts: &[Sort],
        _limit: &Option<Limit>,
        _options: &HashMap<String, String>,
    ) {
        self.tgt_cols = columns.to_vec();
    }

    fn iter_scan(&mut self) -> Option<Row> {
        if self.cnt == 10_000_000 {
            return None;
        }
        self.cnt += 1;
        let mut row = Row::new();
        for tgt_col in &self.tgt_cols {
            match tgt_col.as_str() {
                "id" => row.push("id", Some(Cell::I64(self.cnt as i64))),
                "col" => row.push("col", Some(Cell::String("Hello world".to_string()))),
                _ => {}
            }
        }
        return Some(row);
    }

    fn iter_scan_borrowed(&mut self, row: &mut Row) -> Option<()> {
        if self.cnt == 10_000_000 {
            return None;
        }
        self.cnt += 1;
        for tgt_col in &self.tgt_cols {
            match tgt_col.as_str() {
                "id" => row.push("id", Some(Cell::I64(self.cnt as i64))),
                "col" => row.push("col", Some(Cell::String("Hello world".to_string()))),
                _ => {}
            }
        }
        return Some(());
    }


    fn end_scan(&mut self) {}
}
