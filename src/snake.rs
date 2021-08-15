extern crate rand;
use rand::Rng;


#[derive(Debug, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug)]
pub struct Snake {
    max_width: u32,
    max_height: u32,
    body: Vec<(i32, i32)>,
    pre_direction: Direction,
    direction: Direction,
    food_pos: (i32, i32)
}

impl Snake {
    pub fn new(max_width: u32, max_height: u32) -> Snake {
        let x: i32 = rand::thread_rng().gen_range(10..(max_width - 10)) as i32;
        let y: i32 = rand::thread_rng().gen_range(10..(max_height - 10)) as i32;
        let mut body = Vec::new();
        body.push((x, y));
        // 初始方向为右
        let mut snake = Snake {
            max_width: max_width,
            max_height: max_height,
            body: body,
            direction: Direction::RIGHT,
            pre_direction: Direction::RIGHT,
            food_pos: (0,0),
        };
        snake.generate_food();
        snake
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // 每次更新以更新间隔里最后收到的输入为准
        self.pre_direction = direction;
    }

    pub fn get_body(&self) -> &Vec<(i32, i32)> {
        &self.body
    }

    pub fn take_step(&mut self) {
        // 转向
        if self.check_change_direction_legal(){
            self.direction = self.pre_direction;
        }
        let mut new_pos = self.head_position();
        match self.direction {
            Direction::UP => new_pos.1 -= 1,
            Direction::DOWN => new_pos.1 += 1,
            Direction::LEFT => new_pos.0 -= 1,
            Direction::RIGHT => new_pos.0 += 1,
        }
        self.body.push(new_pos);
        // eat
        if new_pos == self.food_pos{
            self.generate_food();
            return;
        }
        self.body = (&self.body[1..self.body.len()]).to_vec();
    }

    pub fn get_food_pos(&self) -> (i32, i32){
        self.food_pos
    }

    // food must not in body
    fn generate_food(&mut self){
        let mut x: i32 = rand::thread_rng().gen_range(10..(self.max_width - 10)) as i32;
        let mut y: i32 = rand::thread_rng().gen_range(10..(self.max_height - 10)) as i32;
        while self.body.contains(&(x,y)){
            x = rand::thread_rng().gen_range(10..(self.max_width - 10)) as i32;
            y = rand::thread_rng().gen_range(10..(self.max_height - 10)) as i32;
        }
        self.food_pos = (x, y);
    }

    fn head_position(&self) -> (i32, i32) {
        self.body[self.body.len() - 1]
    }

    // 防止 take_step 间方向改了多次
    fn check_change_direction_legal(&self) -> bool{
        // 长度为 1 时不限制
        if self.body.len() == 1{
            return true;
        }
        // 长度大于 1 就不能开倒车
        if matches!(self.direction, Direction::UP) && matches!(self.pre_direction, Direction::DOWN){
            return false;
        }
        if matches!(self.direction, Direction::DOWN) && matches!(self.pre_direction, Direction::UP){
            return false;
        }
        if matches!(self.direction, Direction::LEFT) && matches!(self.pre_direction, Direction::RIGHT){
            return false;
        }
        if matches!(self.direction, Direction::RIGHT) && matches!(self.pre_direction, Direction::LEFT){
            return false;
        }
        true
    } 

    pub fn check_game_over(&self) -> bool{
        let head = self.head_position();
        // 身体相撞
        if (&self.body[0..self.body.len() - 1]).to_vec().contains(&head){
            return true
        }
        // 撞墙
        if head.0 < 0 || head.0 >= self.max_width as i32{
            return true
        }
        if head.1 < 0 || head.1 >= self.max_height as i32{
            return true
        }
        return false
    }
}