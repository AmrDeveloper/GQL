use crate::value::Value;

pub struct Row {
    pub values: Vec<Value>,
}

pub struct Group {
    pub rows: Vec<Row>,
}

impl Group {
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

pub struct GitQLGroups {
    pub titles: Vec<String>,
    pub groups: Vec<Group>,
}

impl GitQLGroups {
    pub fn title_index(&self, title: &str) -> Option<usize> {
        for (index, value) in self.titles.iter().enumerate() {
            if value.eq(title) {
                return Some(index);
            }
        }
        None
    }

    pub fn flat(&mut self) {
        let mut rows: Vec<Row> = vec![];
        for group in &mut self.groups {
            rows.append(&mut group.rows);
        }

        self.groups.clear();
        self.groups.push(Group { rows })
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }
}
