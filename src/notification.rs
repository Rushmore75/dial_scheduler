pub struct Notification<'a> {
    fire_time: u64,
    msg: &'a str
}

impl<'a> Notification<'a> {
    pub fn new(fire_time: u64, msg: &'a str) -> Self {
        Self { fire_time, msg }
    }

    pub fn get_fire_time(&self) -> u64 {
        self.fire_time
    }

    pub fn get_msg(&self) -> &'a str {
        &self.msg
    }

}
