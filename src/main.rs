
struct Files {
    path_stack: Vec<String>,
    path_name: Vec<String>,
    err: Option<String>
}

impl Files {
    pub fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["./".to_string()],
            path_name: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    pub fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();

        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error has occured: {:?}", err);
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };

        let collected = paths.collect::<Vec<_>>();

        self.clear_err();
        self.path_name.clear();

        for path in collected {
            self.path_name
                .push(path.unwrap().path().display().to_string());
        }
    }
    #[allow(dead_code)]
    pub fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }
    #[allow(dead_code)]
    pub fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_name[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }
    #[allow(dead_code)]
    pub fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    pub fn clear_err(&mut self) {
        self.err = None;
    }
}

fn main() {
    let mut fs = String::new();

    std::io::stdin()
        .read_line(&mut fs)
        .expect("fail");
    
    let _dir = fs;

    let dir = Files::new().path_name;

    println!("{:?}", dir);
}