use clap::{App, Arg};
use libc::usleep;
use ncurses::constants::*;
use ncurses::CURSOR_VISIBILITY::*;
use ncurses::*;
use nix::sys::signal::{signal, SigHandler, Signal};

fn my_mvaddstr(y: i32, mut x: i32, string: &str) -> i32 {
    let mut start = 0;
    while x < 0 {
        x += 1;
        start += 1;
        if start > string.len() {
            return ERR;
        }
    }
    for ch in string[start..].chars() {
        if mvaddch(y, x, ch as u32) == ERR {
            return ERR;
        }
        x += 1;
    }
    OK
}

fn add_smoke(y: i32, x: i32) {
    const SMOKEPTNS: usize = 16;
    #[derive(Debug, Clone, Copy)]
    struct Smokes {
        y: i32,
        x: i32,
        ptrn: i32,
        kind: i32,
    }
    static mut S: [Smokes; 1000] = [Smokes {
        y: 0,
        x: 0,
        ptrn: 0,
        kind: 0,
    }; 1000];
    static mut SUM: i32 = 0;
    const SMOKE: [[&str; SMOKEPTNS]; 2] = [
        [
            "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O",
            "O", "O", "O", "O", " ",
        ],
        [
            "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@",
            "@", "@", "@", "@", " ",
        ],
    ];
    const ERASER: [&str; SMOKEPTNS] = [
        "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ",
        " ", " ", " ", " ",
    ];
    static mut DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    static mut DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];
    if x % 4 == 0 {
        for i in 0..unsafe { SUM } {
            unsafe {
                my_mvaddstr(
                    S[i as usize].y,
                    S[i as usize].x,
                    ERASER[S[i as usize].ptrn as usize],
                );
                S[i as usize].y -= DY[S[i as usize].ptrn as usize];
                S[i as usize].x += DX[S[i as usize].ptrn as usize];
                S[i as usize].ptrn += if S[i as usize].ptrn < SMOKEPTNS as i32 - 1 {
                    1
                } else {
                    0
                };
                my_mvaddstr(
                    S[i as usize].y,
                    S[i as usize].x,
                    SMOKE[S[i as usize].kind as usize][S[i as usize].ptrn as usize],
                );
            }
        }
        unsafe {
            my_mvaddstr(y, x, SMOKE[(SUM % 2) as usize][0]);
            S[SUM as usize].y = y;
            S[SUM as usize].x = x;
            S[SUM as usize].ptrn = 0;
            S[SUM as usize].kind = SUM % 2;
            SUM += 1;
        }
    }
}

fn add_man(y: i32, x: i32) {
    const MAN: [[&str; 2]; 2] = [["", "(O)"], ["Help!", "\\O/"]];
    for i in 0..2 {
        my_mvaddstr(
            y + i,
            x,
            MAN[((LOGO_LENGTH + x) / 12 % 2) as usize][i as usize],
        );
    }
}

fn add_sl(x: i32, is_fly: bool, is_accident: bool) -> i32 {
    const SL: [[&str; (LOGO_HIGHT + 1) as usize]; LOGO_HIGHT as usize] = [
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_11, LWHL_12, DEL_LN],
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_21, LWHL_22, DEL_LN],
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_31, LWHL_32, DEL_LN],
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_41, LWHL_42, DEL_LN],
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_51, LWHL_52, DEL_LN],
        [LOGO_1, LOGO_2, LOGO_3, LOGO_4, LWHL_61, LWHL_62, DEL_LN],
    ];
    const COAL: [&str; (LOGO_HIGHT + 1) as usize] =
        [LCOAL_1, LCOAL_2, LCOAL_3, LCOAL_4, LCOAL_5, LCOAL_6, DEL_LN];
    const CAR: [&str; (LOGO_HIGHT + 1) as usize] =
        [LCAR_1, LCAR_2, LCAR_3, LCAR_4, LCAR_5, LCAR_6, DEL_LN];
    if x < -LOGO_LENGTH {
        return ERR;
    }

    let mut y: i32 = LINES() / 2 - 3;
    let mut py1: i32 = 0;
    let mut py2: i32 = 0;
    let mut py3: i32 = 0;

    if is_fly {
        y = (x / 6) + LINES() - (COLS() / 6) - LOGO_HIGHT;
        py1 = 2;
        py2 = 4;
        py3 = 6;
    }
    for i in 0..(LOGO_HIGHT + 1) {
        my_mvaddstr(
            y + i,
            x,
            SL[((LOGO_LENGTH + x) / 3 % LOGO_PATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + py1, x + 21, COAL[i as usize]);
        my_mvaddstr(y + i + py2, x + 42, CAR[i as usize]);
        my_mvaddstr(y + i + py3, x + 63, CAR[i as usize]);
    }
    if is_accident {
        add_man(y + 1, x + 14);
        add_man(y + 1 + py2, x + 45);
        add_man(y + 1 + py2, x + 53);
        add_man(y + 1 + py3, x + 66);
        add_man(y + 1 + py3, x + 74);
    }
    add_smoke(y - 1, x + LOGO_FUNNEL);
    OK
}

fn add_c51(x: i32, is_fly: bool, is_accident: bool) -> i32 {
    const C51: [[&str; C51_HIGHT as usize + 1]; C51_PATTERNS as usize] = [
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_11,
            C51_WH_12, C51_WH_13, C51_WH_14, C51_DEL,
        ],
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_21,
            C51_WH_22, C51_WH_23, C51_WH_24, C51_DEL,
        ],
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_31,
            C51_WH_32, C51_WH_33, C51_WH_34, C51_DEL,
        ],
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_41,
            C51_WH_42, C51_WH_43, C51_WH_44, C51_DEL,
        ],
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_51,
            C51_WH_52, C51_WH_53, C51_WH_54, C51_DEL,
        ],
        [
            C51_STR_1, C51_STR_2, C51_STR_3, C51_STR_4, C51_STR_5, C51_STR_6, C51_STR_7, C51_WH_61,
            C51_WH_62, C51_WH_63, C51_WH_64, C51_DEL,
        ],
    ];
    const COAL: [&str; C51_HIGHT as usize + 1] = [
        COAL_DEL, COAL_01, COAL_02, COAL_03, COAL_04, COAL_05, COAL_06, COAL_07, COAL_08, COAL_09,
        COAL_10, COAL_DEL,
    ];
    let mut y;
    let mut dy: i32 = 0;

    if x < -C51_LENGTH {
        return ERR;
    }
    y = LINES() / 2 - 5;

    if is_fly {
        y = (x / 7) + LINES() - COLS() / 7 - C51_HIGHT;
        dy = 1;
    }
    for i in 0..(C51_HIGHT as i32 + 1) {
        my_mvaddstr(
            y + i,
            x,
            C51[((C51_LENGTH + x) % C51_PATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + dy, x + 55, COAL[i as usize]);
    }
    if is_accident {
        add_man(y + 3, x + 45);
        add_man(y + 3, x + 49);
    }
    add_smoke(y - 1, x + C51_FUNNEL);
    OK
}

fn add_d51(x: i32, is_fly: bool, is_accident: bool) -> i32 {
    const D51: [[&str; (D51_HIGHT + 1) as usize]; D51_PATTERNS as usize] = [
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_11, D51_WHL_12, D51_WHL_13, D51_DEL,
        ],
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_21, D51_WHL_22, D51_WHL_23, D51_DEL,
        ],
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_31, D51_WHL_32, D51_WHL_33, D51_DEL,
        ],
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_41, D51_WHL_42, D51_WHL_43, D51_DEL,
        ],
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_51, D51_WHL_52, D51_WHL_53, D51_DEL,
        ],
        [
            D51_STR_1, D51_STR_2, D51_STR_3, D51_STR_4, D51_STR_5, D51_STR_6, D51_STR_7,
            D51_WHL_61, D51_WHL_62, D51_WHL_63, D51_DEL,
        ],
    ];
    const COAL: [&str; (D51_HIGHT + 1) as usize] = [
        COAL_01, COAL_02, COAL_03, COAL_04, COAL_05, COAL_06, COAL_07, COAL_08, COAL_09, COAL_10,
        COAL_DEL,
    ];
    let mut y;
    let mut dy: i32 = 0;
    if x < -D51_LENGTH {
        return ERR;
    }
    y = LINES() / 2 - 5;
    if is_fly {
        y = (x / 7) + LINES() - COLS() / 7 - D51_HIGHT;
        dy = 1;
    }
    for i in 0..(D51_HIGHT as i32 + 1) {
        my_mvaddstr(
            y + i,
            x,
            D51[((D51_LENGTH + x) % D51_PATTERNS) as usize][i as usize],
        );
        my_mvaddstr(y + i + dy, x + 53, COAL[i as usize]);
    }
    if is_accident {
        add_man(y + 2, x + 43);
        add_man(y + 2, x + 47);
    }
    add_smoke(y - 1, x + D51_FUNNEL);
    OK
}

const D51_HIGHT: i32 = 10;
const D51_FUNNEL: i32 = 7;
const D51_LENGTH: i32 = 83;
const D51_PATTERNS: i32 = 6;

const D51_STR_1: &str = "      ====        ________                ___________ ";
const D51_STR_2: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
const D51_STR_3: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
const D51_STR_4: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
const D51_STR_5: &str = "  |      |  |   H  |__--------------------| [___] |   ";
const D51_STR_6: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
const D51_STR_7: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

const D51_WHL_11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51_WHL_12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51_WHL_13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

const D51_WHL_21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51_WHL_22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
const D51_WHL_23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51_WHL_31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
const D51_WHL_32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51_WHL_33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51_WHL_41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
const D51_WHL_42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51_WHL_43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51_WHL_51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51_WHL_52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
const D51_WHL_53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51_WHL_61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51_WHL_62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51_WHL_63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

const D51_DEL: &str = "                                                      ";

const COAL_01: &str = "                              ";
const COAL_02: &str = "                              ";
const COAL_03: &str = "    _________________         ";
const COAL_04: &str = "   _|                \\_____A  ";
const COAL_05: &str = " =|                        |  ";
const COAL_06: &str = " -|                        |  ";
const COAL_07: &str = "__|________________________|_ ";
const COAL_08: &str = "|__________________________|_ ";
const COAL_09: &str = "   |_D__D__D_|  |_D__D__D_|   ";
const COAL_10: &str = "    \\_/   \\_/    \\_/   \\_/    ";

const COAL_DEL: &str = "                              ";

const LOGO_HIGHT: i32 = 6;
const LOGO_FUNNEL: i32 = 4;
const LOGO_LENGTH: i32 = 84;
const LOGO_PATTERNS: i32 = 6;

const LOGO_1: &str = "     ++      +------ ";
const LOGO_2: &str = "     ||      |+-+ |  ";
const LOGO_3: &str = "   /---------|| | |  ";
const LOGO_4: &str = "  + ========  +-+ |  ";

const LWHL_11: &str = " _|--O========O~\\-+  ";
const LWHL_12: &str = "//// \\_/      \\_/    ";

const LWHL_21: &str = " _|--/O========O\\-+  ";
const LWHL_22: &str = "//// \\_/      \\_/    ";

const LWHL_31: &str = " _|--/~O========O-+  ";
const LWHL_32: &str = "//// \\_/      \\_/    ";

const LWHL_41: &str = " _|--/~\\------/~\\-+  ";
const LWHL_42: &str = "//// \\_O========O    ";

const LWHL_51: &str = " _|--/~\\------/~\\-+  ";
const LWHL_52: &str = "//// \\O========O/    ";

const LWHL_61: &str = " _|--/~\\------/~\\-+  ";
const LWHL_62: &str = "//// O========O_/    ";

const LCOAL_1: &str = "____                 ";
const LCOAL_2: &str = "|   \\@@@@@@@@@@@     ";
const LCOAL_3: &str = "|    \\@@@@@@@@@@@@@_ ";
const LCOAL_4: &str = "|                  | ";
const LCOAL_5: &str = "|__________________| ";
const LCOAL_6: &str = "   (O)       (O)     ";

const LCAR_1: &str = "____________________ ";
const LCAR_2: &str = "|  ___ ___ ___ ___ | ";
const LCAR_3: &str = "|  |_| |_| |_| |_| | ";
const LCAR_4: &str = "|__________________| ";
const LCAR_5: &str = "|__________________| ";
const LCAR_6: &str = "   (O)        (O)    ";

const DEL_LN: &str = "                     ";

const C51_HIGHT: i32 = 11;
const C51_FUNNEL: i32 = 7;
const C51_LENGTH: i32 = 87;
const C51_PATTERNS: i32 = 6;

const C51_DEL: &str = "                                                       ";

const C51_STR_1: &str = "        ___                                            ";
const C51_STR_2: &str = "       _|_|_  _     __       __             ___________";
const C51_STR_3: &str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
const C51_STR_4: &str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
const C51_STR_5: &str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
const C51_STR_6: &str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
const C51_STR_7: &str = "|    | _______|_::-----------------[][]-----|       |  ";

const C51_WH_61: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_62: &str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__";
const C51_WH_63: &str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     ";
const C51_WH_64: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51_WH_51: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_52: &str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__";
const C51_WH_53: &str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     ";
const C51_WH_54: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51_WH_41: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_42: &str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__";
const C51_WH_43: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51_WH_44: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51_WH_31: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_32: &str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__";
const C51_WH_33: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51_WH_34: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51_WH_21: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_22: &str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__";
const C51_WH_23: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51_WH_24: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51_WH_11: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51_WH_12: &str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__";
const C51_WH_13: &str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     ";
const C51_WH_14: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

fn main() {
    let matches = App::new("sl")
        .version("0.1")
        .arg(Arg::with_name("accident").short("a").long("accident"))
        .arg(Arg::with_name("fly").short("f").long("fly"))
        .arg(Arg::with_name("logo").short("l").long("logo"))
        .arg(Arg::with_name("c51").short("c").long("c51"))
        .arg(Arg::with_name("intr").short("e").long("intr"))
        .get_matches();
    let is_accident = matches.is_present("accident");
    let is_logo = matches.is_present("logo");
    let is_c51 = matches.is_present("c51");
    let is_fly = matches.is_present("fly");
    let is_intr = matches.is_present("intr");
    unsafe {
        if !is_intr {
            signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
        }
    }
    initscr();
    noecho();
    curs_set(CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    leaveok(stdscr(), true);
    scrollok(stdscr(), false);
    let mut x = COLS() - 1;
    loop {
        if is_logo {
            if add_sl(x, is_fly, is_accident) == ERR {
                break;
            }
        } else if is_c51 {
            if add_c51(x, is_fly, is_accident) == ERR {
                break;
            }
        } else {
            if add_d51(x, is_fly, is_accident) == ERR {
                break;
            }
        }
        x -= 1;
        getch();
        refresh();
        unsafe {
            usleep(40000);
        }
    }
    mvcur(0, COLS() - 1, LINES() - 1, 0);
    endwin();
}
