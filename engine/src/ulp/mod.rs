//! SwiftyULP-style ULP services (ported from uplKit / Go reference).



mod archive;

mod buckets;

mod clean;

mod cred;

mod extract;

mod input;

mod misc;

mod sort;

mod sort_country;

mod sort_keyword;

mod swifty_data;



pub use archive::{extract_archive, is_archive_path, materialize_input};

pub use buckets::{Buckets, SortStats};

pub use clean::{CleanOp, CleanStats};

pub use cred::{parse_line, parse_login_pass, parse_ulp_line, Cred};

pub use extract::{ExtractFormat, ExtractStats};

pub use misc::{MiscOp, MiscStats};

pub use sort::sort_ulp;

pub use sort_country::{sort_by_country, SortCountryStats, tld_of};

pub use sort_keyword::{search_ulp, sort_by_keyword, SortKeywordStats};



use crate::job_control::JobControl;



#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]

pub enum UlpTool {

    #[default]

    Sort,

    SortCountry,

    SortKeyword,

    Search,

    ExtractUrlLoginPass,

    ExtractLoginPass,

    ExtractUserPass,

    CleanDedupe,

    CleanEmptyLines,

    CleanJunk,

    CleanBlacklist,

    CleanEmptyChars,

    CleanWeak,

    CleanProtocols,

    CleanCapture,

    MiscMerge,

    MiscSplit,

    MiscFilter,

}



impl UlpTool {

    pub const ALL: [UlpTool; 18] = [

        Self::Sort,

        Self::SortCountry,

        Self::SortKeyword,

        Self::Search,

        Self::ExtractUrlLoginPass,

        Self::ExtractLoginPass,

        Self::ExtractUserPass,

        Self::CleanDedupe,

        Self::CleanEmptyLines,

        Self::CleanJunk,

        Self::CleanBlacklist,

        Self::CleanEmptyChars,

        Self::CleanWeak,

        Self::CleanProtocols,

        Self::CleanCapture,

        Self::MiscMerge,

        Self::MiscSplit,

        Self::MiscFilter,

    ];



    pub fn label(self, lang: crate::i18n::Lang) -> &'static str {

        let t = crate::i18n::tr(lang);

        match self {

            Self::Sort => t.ulp_tool_sort,

            Self::SortCountry => t.ulp_tool_sort_country,

            Self::SortKeyword => t.ulp_tool_sort_keyword,

            Self::Search => t.ulp_tool_search,

            Self::ExtractUrlLoginPass => t.ulp_tool_extract_ulp,

            Self::ExtractLoginPass => t.ulp_tool_extract_lp,

            Self::ExtractUserPass => t.ulp_tool_extract_up,

            Self::CleanDedupe => t.ulp_tool_clean_dedupe,

            Self::CleanEmptyLines => t.ulp_tool_clean_empty,

            Self::CleanJunk => t.ulp_tool_clean_junk,

            Self::CleanBlacklist => t.ulp_tool_clean_blacklist,

            Self::CleanEmptyChars => t.ulp_tool_clean_chars,

            Self::CleanWeak => t.ulp_tool_clean_weak,

            Self::CleanProtocols => t.ulp_tool_clean_proto,

            Self::CleanCapture => t.ulp_tool_clean_capture,

            Self::MiscMerge => t.ulp_tool_misc_merge,

            Self::MiscSplit => t.ulp_tool_misc_split,

            Self::MiscFilter => t.ulp_tool_misc_filter,

        }

    }



    pub fn needs_output_dir(self) -> bool {

        matches!(self, Self::Sort | Self::SortCountry | Self::SortKeyword)

    }



    pub fn needs_keywords(self) -> bool {

        matches!(

            self,

            Self::SortKeyword

                | Self::Search

                | Self::ExtractUrlLoginPass

                | Self::ExtractLoginPass

                | Self::ExtractUserPass

                | Self::MiscFilter

        )

    }



    pub fn clean_op(self) -> Option<CleanOp> {

        match self {

            Self::CleanDedupe => Some(CleanOp::Dedupe),

            Self::CleanEmptyLines => Some(CleanOp::EmptyLines),

            Self::CleanJunk => Some(CleanOp::Junk),

            Self::CleanBlacklist => Some(CleanOp::Blacklist),

            Self::CleanEmptyChars => Some(CleanOp::EmptyChars),

            Self::CleanWeak => Some(CleanOp::Weak),

            Self::CleanProtocols => Some(CleanOp::Protocols),

            Self::CleanCapture => Some(CleanOp::Capture),

            _ => None,

        }

    }



    pub fn extract_format(self) -> Option<ExtractFormat> {

        match self {

            Self::ExtractUrlLoginPass => Some(ExtractFormat::UrlLoginPass),

            Self::ExtractLoginPass => Some(ExtractFormat::LoginPass),

            Self::ExtractUserPass => Some(ExtractFormat::UserPass),

            _ => None,

        }

    }



    pub fn misc_op(self) -> Option<MiscOp> {

        match self {

            Self::MiscMerge => Some(MiscOp::Merge),

            Self::MiscSplit => Some(MiscOp::Split),

            Self::MiscFilter => Some(MiscOp::Filter),

            _ => None,

        }

    }

}



#[derive(Clone, Debug, Default)]

pub struct UlpJobSummary {

    pub message: String,

}



pub fn run_tool(

    tool: UlpTool,

    input: &str,

    output: &str,

    output_dir: &str,

    keywords: &[String],

    control: Option<&JobControl>,

) -> anyhow::Result<UlpJobSummary> {

    let msg = match tool {

        UlpTool::Sort => sort_ulp(input, output_dir, control)?.message,

        UlpTool::SortCountry => sort_by_country(input, output_dir, control)?.message,

        UlpTool::SortKeyword => sort_by_keyword(input, output_dir, keywords, control)?.message,

        UlpTool::Search => search_ulp(input, output, keywords, control)?.message,

        UlpTool::ExtractUrlLoginPass | UlpTool::ExtractLoginPass | UlpTool::ExtractUserPass => {

            let fmt = tool.extract_format().unwrap();

            extract::extract_swifty(input, output, fmt, keywords, control)?.message

        }

        UlpTool::CleanDedupe

        | UlpTool::CleanEmptyLines

        | UlpTool::CleanJunk

        | UlpTool::CleanBlacklist

        | UlpTool::CleanEmptyChars

        | UlpTool::CleanWeak

        | UlpTool::CleanProtocols

        | UlpTool::CleanCapture => {

            let op = tool.clean_op().unwrap();

            clean::run_clean(op, input, output, control)?.message

        }

        UlpTool::MiscMerge | UlpTool::MiscSplit | UlpTool::MiscFilter => {

            let op = tool.misc_op().unwrap();

            let filter = keywords.first().map(String::as_str).unwrap_or("");

            misc::run_misc(op, input, output, filter, 100_000, control)?.message

        }

    };

    Ok(UlpJobSummary { message: msg })

}



#[cfg(test)]

mod tests {

    use super::*;

    use std::fs;



    #[test]

    fn sort_writes_bucket_files() {

        let dir = std::env::temp_dir().join("lhf_ulp_sort_test");

        let _ = fs::remove_dir_all(&dir);

        fs::create_dir_all(&dir).unwrap();

        let input = dir.join("in.txt");

        fs::write(

            &input,

            "user@gmail.com:pass1\nhttps://x.com:john:secret\n",

        )

        .unwrap();

        let out = dir.join("out");

        let stats = sort_ulp(&input.display().to_string(), &out.display().to_string(), None).unwrap();

        assert!(stats.mail_pass >= 1);

        assert!(out.join("Mail Pass.txt").exists());

        let _ = fs::remove_dir_all(&dir);

    }



    #[test]

    fn sort_country_writes_tld_files() {

        let dir = std::env::temp_dir().join("lhf_ulp_country_test");

        let _ = fs::remove_dir_all(&dir);

        fs::create_dir_all(&dir).unwrap();

        let input = dir.join("in.txt");

        fs::write(&input, "user@gmail.com:pass1\nuser@mail.de:pass2\n").unwrap();

        let out = dir.join("out");

        let stats =

            sort_by_country(&input.display().to_string(), &out.display().to_string(), None).unwrap();

        assert!(stats.tld_buckets >= 1);

        assert!(out.join("by_tld").exists());

        let _ = fs::remove_dir_all(&dir);

    }



    #[test]

    fn sort_keyword_buckets() {

        let dir = std::env::temp_dir().join("lhf_ulp_kw_test");

        let _ = fs::remove_dir_all(&dir);

        fs::create_dir_all(&dir).unwrap();

        let input = dir.join("in.txt");

        fs::write(&input, "user@gmail.com:pass1\nuser@paypal.com:x\n").unwrap();

        let out = dir.join("out");

        let stats = sort_by_keyword(

            &input.display().to_string(),

            &out.display().to_string(),

            &["gmail.com".to_string()],

            None,

        )

        .unwrap();

        assert!(stats.matched_lines >= 1);

        assert!(out.join("gmail.com.txt").exists());

        let _ = fs::remove_dir_all(&dir);

    }

}


