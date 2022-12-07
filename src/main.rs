use std::io;

const BOARD_SIZE: usize = 15;

struct Game {
    board: [[i8; BOARD_SIZE]; BOARD_SIZE],
    turn: i8,
}

impl Game {
    fn new() -> Game {
        // 初始化游戏状态
        let mut game = Game {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            turn: 1,
        };

        // 初始化棋盘
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                game.board[i][j] = 0;
            }
        }

        game
    }

    fn play(&mut self, x: usize, y: usize) {
        // 如果玩家重复落子，则重新询问玩家落子点
        if self.board[x][y] != 0 {
            println!("Player {} repeated playing on the same spot!", self.turn);
            let mut x = String::new();
            let mut y = String::new();
            print!("Please enter a new x: ");
            io::stdin().read_line(&mut x).unwrap();
            print!("Please enter a new y: ");
            io::stdin().read_line(&mut y).unwrap();
            let x: usize = x.trim().parse().unwrap();
            let y: usize = y.trim().parse().unwrap();
            self.play(x, y);
            return;
        }

        // 记录当前玩家下棋
        self.board[x][y] = self.turn;

        // 切换玩家
        self.turn = if self.turn == 1 { 2 } else { 1 };
    }

    fn winner(&self) -> i8 {
        // 检查是否有玩家获胜
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let player = self.board[i][j];
                if player == 0 {
                    continue;
                }

                if (i + 4 < BOARD_SIZE) &&
                    self.board[i + 1][j] == player &&
                    self.board[i + 2][j] == player &&
                    self.board[i + 3][j] == player &&
                    self.board[i + 4][j] == player
                {
                    return player;
                }

                if (j + 4 < BOARD_SIZE) &&
                    self.board[i][j + 1] == player &&
                    self.board[i][j + 2] == player &&
                    self.board[i][j + 3] == player &&
                    self.board[i][j + 4] == player
                {
                    return player;
                }

                if (i + 4 < BOARD_SIZE) && (j + 4 < BOARD_SIZE) &&
                    self.board[i + 1][j + 1] == player &&
                    self.board[i + 2][j + 2] == player &&
                    self.board[i + 3][j + 3] == player &&
                    self.board[i + 4][j + 4] == player
                {
                    return player;
                }

                if (i >= 4) && (j + 4 < BOARD_SIZE) &&
                    self.board[i - 1][j + 1] == player &&
                    self.board[i - 2][j + 2] == player &&
                    self.board[i - 3][j + 3] == player &&
                    self.board[i - 4][j + 4] == player
                {
                    return player;
                }
            }
        }
        0
    }

    fn is_game_over(&self) -> bool {
        // 检查游戏是否已结束
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let mut game = Game::new();
    while !game.is_game_over() {
        // 询问玩家下棋
        let mut x = String::new();
        let mut y = String::new();
        println!("Player {}'s turn:", game.turn);
        print!("x: ");
        io::stdin().read_line(&mut x).unwrap();
        print!("y: ");
        io::stdin().read_line(&mut y).unwrap();
        let x: usize = x.trim().parse().unwrap();
        let y: usize = y.trim().parse().unwrap();

    
        // 下棋
        game.play(x, y);
    
        // 检查是否有玩家胜出
        let winner = game.winner();
        if winner != 0 {
            println!("Player {} wins!", winner);
            break;
        }
    }
}
