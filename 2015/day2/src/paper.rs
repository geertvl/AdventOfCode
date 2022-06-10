pub fn calculate_wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
    let area_l_w = 2*l*w;
    let area_w_h = 2*w*h;
    let area_h_l = 2*h*l;
    let surface = area_l_w + area_w_h + area_h_l;
    let extra = vec![l*w, w * h, h * l]
        .iter()
        .min()
        .unwrap()
        .clone();
    surface + extra
}

pub fn calculate_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let ribbon = vec![
        2*l+2*w,
        2*w+2*h,
        2*h+2*l,
    ]
    .iter()
    .min()
    .unwrap()
    .clone();

    let bow = l*w*h;

    ribbon + bow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_2x3x4_58() {
        let paper = calculate_wrapping_paper(2, 3, 4);
        assert_eq!(paper, 58);
    }

    #[test]
    fn present_1x1x10_43() {
        let paper = calculate_wrapping_paper(1, 1, 10);
        assert_eq!(paper, 43);
    }

    #[test]
    fn ribbon_2x3x4_34() {
        let ribbon = calculate_ribbon(2, 3, 4);
        assert_eq!(ribbon, 34);
    }

    #[test]
    fn ribbon_1x1x10_14() {
        let ribbon = calculate_ribbon(1, 1, 10);
        assert_eq!(ribbon, 14);
    }
}