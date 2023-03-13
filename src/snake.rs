use std::collections::LinkedList;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>, // apple을 먹을때 사용할 예정.
}

/// private methods
impl Snake {
    fn head(&self) -> Block {
        *self.body.front().unwrap()
    }

    fn next_head(&self) -> Block {
        let (x, y) = self.head_position();
        match self.direction {
            Direction::Up => Block { x, y: y - 1 },
            Direction::Down => Block { x, y: y + 1 },
            Direction::Left => Block { x: x - 1, y },
            Direction::Right => Block { x: x + 1, y },
        }
    }
}

/// public methods
impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y }); // little tails
        body.push_back(Block { x: x + 1, y }); // little tails
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.head();
        (head.x, head.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head_position(&self) -> (i32, i32) {
        let next_head = self.next_head();
        (next_head.x, next_head.y)
    }

    ///  head_direction이 가리키는 방향으로 한 칸 전진한다.
    ///
    ///  @TODO 메모리 풀을 만들면 매번 새 Block을 만들지 않아도 될듯
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        self.direction = dir.unwrap_or(self.direction);
        self.body.push_front(self.next_head());
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    /// make it tail grow
    pub fn grow_tail(&mut self) {
        if let Some(block) = self.tail {
            self.body.push_back(block);
        }
    }

    /// 종료조건 중 하나
    /// @TODO: 종료조건을 한 군데에서 모아보는게 더 낫지 않나?
    pub fn overlap(&self) -> bool {
        let head = self.head();
        self.body.iter().any(|block| *block == head)
    }

    pub fn overlap_with(&self, x: i32, y: i32) -> bool {
        let tmp = Block { x, y };
        self.body.iter().any(|block| *block == tmp)
    }

    pub fn for_each<F: FnMut(i32, i32)>(&self, mut f: F) {
        self.body.iter().for_each(|block| f(block.x, block.y));
    }
}
