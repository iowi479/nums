pub struct Output {}

impl Output {
    pub fn number_string(num: u64) -> String {
        let mut lines = vec![String::from("     "); 6];

        lines[0] += "                ______     _     _         ";
        lines[1] += "               |___  /    | |   | |  _     ";
        lines[2] += "                  / / __ _| |__ | | (_)    ";
        lines[3] += "                 / / / _` | '_ \\| |        ";
        lines[4] += "                / /_| (_| | | | | |  _     ";
        lines[5] += "               /_____\\__,_|_| |_|_| (_)    ";

        let d = Self::digits_string(num);
        let mut s = d.split("\n");
        for i in 0..6 {
            lines[i] += s.next().unwrap();
        }

        return lines.join("\n");
    }

    pub fn digits_string(num: u64) -> String {
        let s = num.to_string();
        let mut lines = vec![String::from("     "); 6];

        for c in s.chars() {
            match c {
                '1' => {
                    lines[0] += " __  ";
                    lines[1] += "/_ | ";
                    lines[2] += " | | ";
                    lines[3] += " | | ";
                    lines[4] += " | | ";
                    lines[5] += " |_| ";
                }

                '2' => {
                    lines[0] += " ___   ";
                    lines[1] += "|__ \\  ";
                    lines[2] += "   ) | ";
                    lines[3] += "  / /  ";
                    lines[4] += " / /_  ";
                    lines[5] += "|____| ";
                }

                '3' => {
                    lines[0] += " ____   ";
                    lines[1] += "|___ \\  ";
                    lines[2] += "  __) | ";
                    lines[3] += " |__ <  ";
                    lines[4] += " ___) | ";
                    lines[5] += "|____/  ";
                }

                '4' => {
                    lines[0] += " _  _    ";
                    lines[1] += "| || |   ";
                    lines[2] += "| || |_  ";
                    lines[3] += "|__   _| ";
                    lines[4] += "   | |   ";
                    lines[5] += "   |_|   ";
                }

                '5' => {
                    lines[0] += " _____  ";
                    lines[1] += "| ____| ";
                    lines[2] += "| |__   ";
                    lines[3] += "|___ \\  ";
                    lines[4] += " ___) | ";
                    lines[5] += "|____/  ";
                }

                '6' => {
                    lines[0] += "   __   ";
                    lines[1] += "  / /   ";
                    lines[2] += " / /_   ";
                    lines[3] += "| '_ \\  ";
                    lines[4] += "| (_) | ";
                    lines[5] += " \\___/  ";
                }

                '7' => {
                    lines[0] += " ______  ";
                    lines[1] += "|____  | ";
                    lines[2] += "    / /  ";
                    lines[3] += "   / /   ";
                    lines[4] += "  / /    ";
                    lines[5] += " /_/     ";
                }

                '8' => {
                    lines[0] += "  ___   ";
                    lines[1] += " / _ \\  ";
                    lines[2] += "| (_) | ";
                    lines[3] += " > _ <  ";
                    lines[4] += "| (_) | ";
                    lines[5] += " \\___/  ";
                }

                '9' => {
                    lines[0] += "  ___   ";
                    lines[1] += " / _ \\  ";
                    lines[2] += "| (_) | ";
                    lines[3] += " \\__, | ";
                    lines[4] += "   / /  ";
                    lines[5] += "  /_/   ";
                }

                '0' => {
                    lines[0] += "  ___   ";
                    lines[1] += " / _ \\  ";
                    lines[2] += "| | | | ";
                    lines[3] += "| | | | ";
                    lines[4] += "| |_| | ";
                    lines[5] += " \\___/  ";
                }
                _ => {}
            }
        }

        return lines.join("\n");
    }
    pub fn dices_string(dices: &[u64]) -> String {
        let mut lines = vec![String::from("     "); 6];

        lines[0] += "__          ___   _       __     _         ";
        lines[1] += "\\ \\        / (_) (_)     / _|   | |  _     ";
        lines[2] += " \\ \\  /\\  / / _   _ _ __| |_ ___| | (_)    ";
        lines[3] += "  \\ \\/  \\/ / | | | | '__|  _/ _ \\ |        ";
        lines[4] += "   \\  /\\  /  | |_| | |  | ||  __/ |  _     ";
        lines[5] += "    \\/  \\/    \\__,_|_|  |_| \\___|_| (_)    ";

        for dice in dices {
            let d = Self::digits_string(*dice);
            let mut s = d.split("\n");
            for i in 0..6 {
                lines[i] += s.next().unwrap();
            }
        }

        return lines.join("\n");
    }
}
