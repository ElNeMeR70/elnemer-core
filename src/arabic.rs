//! Arabic NLP utilities: diacritics removal and Hisab Al-Jummal

pub struct ArabicNLP;

impl ArabicNLP {
    /// Removes Arabic vowels/tashkeel (Fatha, Damma, Kasra, Sukun, Tanween, Shadda, Tatweel)
    pub fn strip_tashkeel(text: &str) -> String {
        text.chars()
            .filter(|&c| !matches!(c, '\u{064B}'..='\u{0652}' | '\u{0670}' | '\u{0640}'))
            .collect()
    }

    /// Calculates Hisab Al-Jummal (الأبجد الكبير)
    pub fn jummal_value(text: &str) -> u32 {
        let cleaned = Self::strip_tashkeel(text);
        cleaned.chars().map(|c| match c {
            'ا' | 'أ' | 'إ' | 'آ' | 'ٱ' => 1,
            'ب' => 2,
            'ج' => 3,
            'د' => 4,
            'ه' | 'ة' => 5,
            'و' | 'ؤ' => 6,
            'ز' => 7,
            'ح' => 8,
            'ط' => 9,
            'ي' | 'ى' | 'ئ' => 10,
            'ك' => 20,
            'ل' => 30,
            'م' => 40,
            'ن' => 50,
            'س' => 60,
            'ع' => 70,
            'ف' => 80,
            'ص' => 90,
            'ق' => 100,
            'ر' => 200,
            'ش' => 300,
            'ت' => 400,
            'ث' => 500,
            'خ' => 600,
            'ذ' => 700,
            'ض' => 800,
            'ظ' => 900,
            'غ' => 1000,
            _ => 0,
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_tashkeel() {
        assert_eq!(ArabicNLP::strip_tashkeel("مُحَمَّدٌ"), "محمد");
    }

    #[test]
    fn test_jummal_value() {
        assert_eq!(ArabicNLP::jummal_value("محمد"), 92);
    }
}
