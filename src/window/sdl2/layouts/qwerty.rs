use skulpin::sdl2::keyboard::Keycode;

use crate::window::layouts_shared::{token::Token, unsupported_key};

/// Maps winit keyboard events to Vim tokens
pub fn handle_qwerty_layout(keycode: Keycode, shift: bool) -> Option<Token<'static>> {
    let special = |text| Some(Token::new(text, true, true));
    let normal = |text| Some(Token::new(text, false, true));
    let partial = |text| Some(Token::new(text, false, false));
    match (keycode, shift) {
        (Keycode::Backspace, _) => special("BS"),
        (Keycode::Tab, _) => special("Tab"),
        (Keycode::Return, _) => special("Enter"),
        (Keycode::Escape, _) => special("Esc"),
        (Keycode::Space, _) => normal(" "),
        (Keycode::Exclaim, _) => normal("!"),
        (Keycode::Quotedbl, _) => normal("\""),
        (Keycode::Hash, _) => normal("#"),
        (Keycode::Dollar, _) => normal("$"),
        (Keycode::Percent, _) => normal("%"),
        (Keycode::Ampersand, _) => normal("&"),
        (Keycode::Quote, false) => normal("'"),
        (Keycode::Quote, true) => normal("\""),
        (Keycode::LeftParen, _) => normal("("),
        (Keycode::RightParen, _) => normal(")"),
        (Keycode::Asterisk, _) => normal("*"),
        (Keycode::Plus, _) => normal("+"),
        (Keycode::Comma, false) => normal(","),
        (Keycode::Comma, true) => special("lt"),
        (Keycode::Minus, false) => partial("-"),
        (Keycode::Minus, true) => partial("_"),
        (Keycode::Period, false) => partial("."),
        (Keycode::Period, true) => partial(">"),
        (Keycode::Slash, false) => partial("/"),
        (Keycode::Slash, true) => partial("?"),
        (Keycode::Num0, false) => partial("0"),
        (Keycode::Num0, true) => special(")"),
        (Keycode::Num1, false) => partial("1"),
        (Keycode::Num1, true) => special("!"),
        (Keycode::Num2, false) => partial("2"),
        (Keycode::Num2, true) => partial("@"),
        (Keycode::Num3, false) => partial("3"),
        (Keycode::Num3, true) => partial("#"),
        (Keycode::Num4, false) => partial("4"),
        (Keycode::Num4, true) => partial("$"),
        (Keycode::Num5, false) => partial("5"),
        (Keycode::Num5, true) => partial("%"),
        (Keycode::Num6, false) => partial("6"),
        (Keycode::Num6, true) => partial("^"),
        (Keycode::Num7, false) => partial("7"),
        (Keycode::Num7, true) => partial("&"),
        (Keycode::Num8, false) => partial("8"),
        (Keycode::Num8, true) => partial("*"),
        (Keycode::Num9, false) => partial("9"),
        (Keycode::Num9, true) => partial("("),
        (Keycode::Colon, _) => normal(":"),
        (Keycode::Semicolon, false) => partial(";"),
        (Keycode::Semicolon, true) => partial(":"),
        (Keycode::Less, _) => special("lt"),
        (Keycode::Equals, false) => partial("="),
        (Keycode::Equals, true) => partial("+"),
        (Keycode::Greater, _) => normal("gt"),
        (Keycode::Question, _) => normal("?"),
        (Keycode::At, _) => normal("@"),
        (Keycode::LeftBracket, false) => partial("["),
        (Keycode::LeftBracket, true) => partial("{"),
        (Keycode::Backslash, false) => partial("\\"),
        (Keycode::Backslash, true) => partial("|"),
        (Keycode::RightBracket, false) => partial("]"),
        (Keycode::RightBracket, true) => partial("}"),
        (Keycode::Caret, _) => normal("^"),
        (Keycode::Underscore, _) => normal("_"),
        (Keycode::Backquote, false) => partial("`"),
        (Keycode::Backquote, true) => partial("~"),
        (Keycode::A, _) => normal("a"),
        (Keycode::B, _) => normal("b"),
        (Keycode::C, _) => normal("c"),
        (Keycode::D, _) => normal("d"),
        (Keycode::E, _) => normal("e"),
        (Keycode::F, _) => normal("f"),
        (Keycode::G, _) => normal("g"),
        (Keycode::H, _) => normal("h"),
        (Keycode::I, _) => normal("i"),
        (Keycode::J, _) => normal("j"),
        (Keycode::K, _) => normal("k"),
        (Keycode::L, _) => normal("l"),
        (Keycode::M, _) => normal("m"),
        (Keycode::N, _) => normal("n"),
        (Keycode::O, _) => normal("o"),
        (Keycode::P, _) => normal("p"),
        (Keycode::Q, _) => normal("q"),
        (Keycode::R, _) => normal("r"),
        (Keycode::S, _) => normal("s"),
        (Keycode::T, _) => normal("t"),
        (Keycode::U, _) => normal("u"),
        (Keycode::V, _) => normal("v"),
        (Keycode::W, _) => normal("w"),
        (Keycode::X, _) => normal("x"),
        (Keycode::Y, _) => normal("y"),
        (Keycode::Z, _) => normal("z"),
        (Keycode::Delete, _) => special("Delete"),
        (Keycode::F1, _) => special("F1"),
        (Keycode::F2, _) => special("F2"),
        (Keycode::F3, _) => special("F3"),
        (Keycode::F4, _) => special("F4"),
        (Keycode::F5, _) => special("F5"),
        (Keycode::F6, _) => special("F6"),
        (Keycode::F7, _) => special("F7"),
        (Keycode::F8, _) => special("F8"),
        (Keycode::F9, _) => special("F9"),
        (Keycode::F10, _) => special("F10"),
        (Keycode::F11, _) => special("F11"),
        (Keycode::F12, _) => special("F12"),
        (Keycode::Insert, _) => special("Insert"),
        (Keycode::Home, _) => special("Home"),
        (Keycode::PageUp, _) => special("PageUp"),
        (Keycode::End, _) => special("End"),
        (Keycode::PageDown, _) => special("PageDown"),
        (Keycode::Right, _) => special("Right"),
        (Keycode::Left, _) => special("Left"),
        (Keycode::Down, _) => special("Down"),
        (Keycode::Up, _) => special("Up"),
        (Keycode::KpDivide, _) => special("/"),
        (Keycode::KpMultiply, _) => special("*"),
        (Keycode::KpMinus, _) => special("-"),
        (Keycode::KpPlus, _) => special("+"),
        (Keycode::KpEnter, _) => special("Enter"),
        (Keycode::Kp0, _) => normal("0"),
        (Keycode::Kp1, _) => normal("1"),
        (Keycode::Kp2, _) => normal("2"),
        (Keycode::Kp3, _) => normal("3"),
        (Keycode::Kp4, _) => normal("4"),
        (Keycode::Kp5, _) => normal("5"),
        (Keycode::Kp6, _) => normal("6"),
        (Keycode::Kp7, _) => normal("7"),
        (Keycode::Kp8, _) => normal("8"),
        (Keycode::Kp9, _) => normal("9"),
        (Keycode::KpPeriod, _) => normal("."),
        (Keycode::KpEquals, _) => normal("="),
        (Keycode::F13, _) => special("F13"),
        (Keycode::F14, _) => special("F14"),
        (Keycode::F15, _) => special("F15"),
        (Keycode::F16, _) => special("F16"),
        (Keycode::F17, _) => special("F17"),
        (Keycode::F18, _) => special("F18"),
        (Keycode::F19, _) => special("F19"),
        (Keycode::F20, _) => special("F20"),
        (Keycode::F21, _) => special("F21"),
        (Keycode::F22, _) => special("F22"),
        (Keycode::F23, _) => special("F23"),
        (Keycode::F24, _) => special("F24"),
        (Keycode::KpLeftParen, _) => normal("("),
        (Keycode::KpRightParen, _) => normal("("),
        (Keycode::KpLeftBrace, _) => normal("["),
        (Keycode::KpRightBrace, _) => normal("]"),
        (Keycode::KpTab, _) => special("TAB"),
        (Keycode::KpBackspace, _) => special("BS"),
        (Keycode::KpA, _) => normal("A"),
        (Keycode::KpB, _) => normal("B"),
        (Keycode::KpC, _) => normal("C"),
        (Keycode::KpD, _) => normal("D"),
        (Keycode::KpE, _) => normal("E"),
        (Keycode::KpF, _) => normal("F"),
        (Keycode::KpPower, _) => normal("^"),
        (Keycode::KpPercent, _) => normal("%"),
        (Keycode::KpLess, _) => special("lt"),
        (Keycode::KpGreater, _) => special("gt"),
        (Keycode::KpAmpersand, _) => normal("&"),
        (Keycode::KpVerticalBar, _) => normal("|"),
        (Keycode::KpColon, _) => normal(":"),
        (Keycode::KpHash, _) => normal("#"),
        (Keycode::KpSpace, _) => normal(" "),
        (Keycode::KpAt, _) => normal("@"),
        (Keycode::KpExclam, _) => normal("!"),
        (Keycode::LCtrl, _) => None,
        (Keycode::LShift, _) => None,
        (Keycode::LAlt, _) => None,
        (Keycode::LGui, _) => None,
        (Keycode::RCtrl, _) => None,
        (Keycode::RShift, _) => None,
        (Keycode::RAlt, _) => None,
        (Keycode::RGui, _) => None,
        (keycode, _) => unsupported_key(keycode),
    }
}
