use ncurses::*;

const REGULAR_PAIR  : i16   = 0;
const HIGHLIGHT_PAIR: i16 = 1;

const K_Q    : i32 = 113;
const K_W    : i32 = 119;
const K_S    : i32 = 115;
const K_UP   : i32 = KEY_UP;
const K_DOWN : i32 = KEY_DOWN;
const K_ENTER: i32 = 10;

fn main() {
    initscr();
    raw();
    // Enable Arrows
    keypad(stdscr(), true);
    // No Keyboard output on screen
    noecho();
    // Hide Cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    // Enable terminal colours
    start_color();
    // Create colour pairs
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    // State
    let items = vec!["Manali", "Mayur", "Ganesh"];
    let mut running = true;
    let mut row = 0;
    let mut selection = false;

    while running {
        erase();
        box_(stdscr(),0,0);
        for (index, item) in items.iter().enumerate() {
            mv(index as i32 + 1, 1);
            if index == row {
                attron(COLOR_PAIR(HIGHLIGHT_PAIR));
                addstr(*item);
                attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
            } else {
                attron(COLOR_PAIR(REGULAR_PAIR));
                addstr(*item);
                attroff(COLOR_PAIR(REGULAR_PAIR));
            }
        }

        refresh();

        let c = getch();
        match c {
            K_Q => {
                running = false;
            }
            K_W | K_UP => {
                if row > 0 {
                    row -= 1;
                }
            }
            K_S | K_DOWN => {
                row += 1;
            }
            K_ENTER => {
                running = false;
                selection = true;
            }
            _ => {}
        };
    }

    endwin();

    if selection {
        println!("{}", items[row]);
    }
}
