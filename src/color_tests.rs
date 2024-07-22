use colors::color::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_colors() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(50, 100, 100);
        let result = color1 + color2;
        assert_eq!(result, Color::new(150, 250, 255));
    }

    #[test]
    fn test_mul_colors_within_bounds() {
        let color1 = Color::new(10, 20, 30);
        let result = color1 * 2.0;
        assert_eq!(result, Color::new(20, 40, 60));
    }

    #[test]
    fn test_mul_colors_large_numbers() {
        let color1 = Color::new(100, 150, 200);
        let result = color1 * 10.0;
        assert_eq!(result, Color::new(255, 255, 255)); // Expecting clamped values
    }

    #[test]
    fn test_mul_colors_zero() {
        let color1 = Color::new(100, 150, 200);
        let result = color1 * 0.0;
        assert_eq!(result, Color::new(0, 0, 0)); // Multiplying by zero should result in zero
    }
}