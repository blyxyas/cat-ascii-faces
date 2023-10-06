use tinyrand::{RandRange, Wyrand, Seeded};

pub struct Cats {
    pub cats: &'static [&'static str],
    rand: Wyrand,
}

impl Cats {
    pub fn new() -> Self {
        Self {
            cats: Self::all(),
            rand: Wyrand::seed(std::time::Instant::now().elapsed().as_nanos().try_into().unwrap()),
        }
    }

    pub fn all() -> &'static [&'static str] {
        &[
            "｡＾･ｪ･＾｡",
            "( ͒ ु- •̫̮ – ू ͒)",
            "( ^..^)ﾉ",
            "( =①ω①=)",
            "( =ω=)..nyaa",
            "( =ノωヽ=)",
            "(´; ω ;｀)",
            "(^-人-^)",
            "(^･o･^)ﾉ”",
            "(^・ω・^ )",
            "(^._.^)ﾉ",
            "(^人^)",
            "(・∀・)",
            "(,,◕　⋏　◕,,)",
            "(.=^・ェ・^=)",
            "(｡･ω･｡)",
            "((≡^⚲͜^≡))",
            "((ΦωΦ))",
            "(*^ω^*)",
            "(*✧×✧*)",
            "(*ΦωΦ*)",
            "(⁎˃ᆺ˂)",
            "(ٛ⁎꒪̕ॢ ˙̫ ꒪ٛ̕ॢ⁎)",
            "₍˄·͈༝·͈˄₎◞ ̑̑ෆ⃛",
            "₍˄·͈༝·͈˄₎ฅ˒˒",
            "₍˄ุ.͡˳̫.˄ุ₎ฅ˒˒",
            "(=｀ω´=)",
            "(=｀ェ´=)",
            "（=´∇｀=）",
            "(=^ ◡ ^=)",
            "(=^-ω-^=)",
            "(=^･^=)",
            "(=^･ω･^)y＝",
            "(=^･ω･^=)",
            "(=^･ｪ･^=)",
            "(=^‥^=)",
            "(=･ω･=)",
            "(=;ω;=)",
            "(=;ェ;=)",
            "(=；ェ；=)",
            "(=;ェ;=)",
            "(=；ェ；=)",
            "(=；ｪ；=)",
            "(=‘ｘ‘=)",
            "(=⌒‿‿⌒=)",
            "(=ↀωↀ=)",
            "(=ↀωↀ=)✧",
            "(=①ω①=)",
            "(=ＴェＴ=)",
            "(=ｘェｘ=)",
            "(=ΦｴΦ=)",
            "(ٛ₌டுͩ ˑ̭ டுͩٛ₌)ฅ",
            "(≚ᄌ≚)ℒℴѵℯ❤",
            "(≚ᄌ≚)ƶƵ",
            "(○｀ω´○)",
            "(●ↀωↀ●)",
            "(●ↀωↀ●)✧",
            "(✦థ ｪ థ)",
            "(ↀДↀ)",
            "(ↀДↀ)⁼³₌₃",
            "(ↀДↀ)✧",
            "(๑•ω•́ฅ✧",
            "(๑ↀᆺↀ๑)☄",
            "(๑ↀᆺↀ๑)✧",
            "(p`･ω･´) q",
            "(p`ω´) q",
            "(Φ∇Φ)",
            "(ΦεΦ)",
            "(ΦωΦ)",
            "(ΦёΦ)",
            "(ΦзΦ)",
            "(ฅ`･ω･´)っ=",
            "(ฅ`ω´ฅ)",
            "(ฅ’ω’ฅ)",
            "(ะ`♔´ะ)",
            "(ะ☫ω☫ะ)",
            "(ㅇㅅㅇ❀)",
            "(ノω<。)",
            "(ꀄꀾꀄ)",
            "（三ФÅФ三）",
            "[ΦωΦ]",
            "] ‘͇̂•̩̫’͇̂ ͒)ฅ ﾆｬ❣",
            "＼(=^‥^)/’`",
            "<(*ΦωΦ*)>",
            "<ΦωΦ>",
            "|ΦωΦ|",
            "|ｪ･`｡)･･･　　",
            "~(=^‥^)",
            "~(=^‥^)_旦~",
            "~(=^‥^)/",
            "~(=^‥^)ノ",
            "~□Pヘ(^･ω･^=)~",
            "⊱ฅ•ω•ฅ⊰",
            "└(=^‥^=)┐",
            "✩⃛( ͒ ु•·̫• ू ͒)",
            "❤(´ω｀*)",
            "ヽ(^‥^=ゞ)",
            "ヾ(*ΦωΦ)ﾉ",
            "ヾ(*ФωФ)βyё βyё☆彡",
            "ヾ(=｀ω´=)ノ”",
            "ヽ(=^･ω･^=)丿",
            "ヾ(=ﾟ･ﾟ=)ﾉ",
            "0( =^･_･^)=〇",
            "٩(ↀДↀ)۶",
            "b(=^‥^=)o",
            "d(=^･ω･^=)b",
            "o(^・x・^)o",
            "o(=・ω・=o)",
            "V(=^･ω･^=)v",
            "ლ(=ↀωↀ=)ლ",
            "ლ(●ↀωↀ●)ლ",
            "ฅ ̂⋒ิ ˑ̫ ⋒ิ ̂ฅ",
            "ฅ( ᵕ ω ᵕ )ฅ",
            "ฅ(´-ω-`)ฅ",
            "ฅ(´・ω・｀)ฅ",
            "ฅ(^ω^ฅ)",
            "ฅ(≚ᄌ≚)",
            "ฅ(⌯͒• ɪ •⌯͒)ฅ❣",
            "ฅ⃛(⌯͒꒪ั ˑ̫ ꒪ั ⌯͒) ﾆｬｯ❣",
            "ฅ(●´ω｀●)ฅ",
            "ฅ*•ω•*ฅ♡",
            "ฅ•ω•ฅ",
            "ฅ⊱*•ω•*⊰ฅ",
            "ㅇㅅㅇ",
            "ミ๏ｖ๏彡",
            "ミ◕ฺｖ◕ฺ彡",
            "=＾● ⋏ ●＾=",
            "ฅ^•ﻌ•^ฅ",
            "₍ᵔ·͈༝·͈ᵔ₎",
            "ฅ(⌯͒•̩̩̩́ ˑ̫ •̩̩̩̀⌯͒)ฅ",
            "₍˄·͈༝·͈˄*₎◞ ̑̑",
            "ଲ( ⓛ ω ⓛ *)ଲ",
            "=^._.^= ∫",
            "ଲ(⁃̗̀̂❍⃓ˑ̫❍⃓⁃̠́̂)ଲ",
        ]
    }

    pub fn cat(&mut self) -> &'static str {
        self.cats[self.rand.next_range(0..self.cats.len())]
    }
}
