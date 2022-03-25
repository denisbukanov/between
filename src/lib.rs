pub struct Config {
    pub prefix: String,
    pub suffix: String,
    pub keep_prefix: bool,
    pub keep_suffix: bool,
    pub trim: bool
}

/// Extract data between `opening_tag` and `closing_tag`
pub fn extract_data(source: &String, opening_tag: &str, closing_tag: &str) -> Vec<String> {
    let config = Config{
        prefix: opening_tag.to_string(),
        suffix: closing_tag.to_string(),
        keep_prefix: true,
        keep_suffix: true,
        trim: false
    };
    extract_data_extended(source, config)
}

/// Extract data between `prefix` and `suffix`
pub fn extract_data_extended(source: &String, config: Config) -> Vec<String> {
    let opening_tag_len = config.prefix.len();
    let closing_tag_len = config.suffix.len();
    let prefix = config.prefix.as_str();
    let suffix = config.suffix.as_str();

    let opening_tags: Vec<_> = source.rmatch_indices(prefix).map(|x| x.0).collect();
    let mut closing_tags: Vec<_> = source.match_indices(suffix).map(|x| x.0).collect();

    opening_tags
        .iter()
        .map(|&start_pos| {
            match closing_tags
                .clone()
                .iter()
                .enumerate()
                .find(|(_, &end_pos)| start_pos < end_pos)
            {
                Some((idx, &end_pos)) => {
                    closing_tags.remove(idx);
                    (start_pos, end_pos)
                }
                None => (0, 0),
            }
        })
        .filter(|&start_end| start_end != (0, 0))
        .collect::<Vec<(usize, usize)>>() // I have to do it in order to reverse list
        .iter()
        .map(|&(start, end)| {
            let start = if config.keep_prefix {
                start
            } else {
                start + opening_tag_len
            };
            let end = if config.keep_suffix {
                end + closing_tag_len
            } else {
                end
            };
            source.get(start..end).unwrap().trim().to_string()
        })
        .rev()
        .collect()
}


#[test]
fn test_parser_trees() {
    let src = String::from("<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>");
    let res = crate::extract_data(&src, "<pre>", "</pre>");
    assert_eq!(
        res,
        vec![
            "<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>",
            "<pre>2<pre>33</pre>2</pre>",
            "<pre>33</pre>",
            "<pre>44</pre>"
        ]
    );
}

#[test]
fn test_parser_trees_keep_all() {
    let config = Config{
        prefix: String::from("<pre>"),
        suffix: String::from("</pre>"),
        keep_prefix: true,
        keep_suffix: true,
        trim: false
    };
    let src = String::from("<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>");
    let res = crate::extract_data_extended(&src, config);
    assert_eq!(
        res,
        vec![
            "<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>",
            "<pre>2<pre>33</pre>2</pre>",
            "<pre>33</pre>",
            "<pre>44</pre>"
        ]
    );
}

#[test]
fn test_parser_trees_stripped() {
    let config = Config{
        prefix: String::from("<pre>"),
        suffix: String::from("</pre>"),
        keep_prefix: false,
        keep_suffix: false,
        trim: false
    };
    let src = String::from("<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>");
    let res = crate::extract_data_extended(&src, config);
    assert_eq!(
        res,
        vec![
            "1<pre>2<pre>33</pre>2</pre><pre>44</pre>1",
            "2<pre>33</pre>2",
            "33",
            "44"
        ]
    );
}

#[test]
fn test_parser_trees_opening() {
    let config = Config{
        prefix: String::from("<pre>"),
        suffix: String::from("</pre>"),
        keep_prefix: true,
        keep_suffix: false,
        trim: false
    };
    let src = String::from("<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>");
    let res = crate::extract_data_extended(&src, config);
    assert_eq!(
        res,
        vec![
            "<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1",
            "<pre>2<pre>33</pre>2",
            "<pre>33",
            "<pre>44"
        ]
    );
}


#[test]
fn test_parser_trees_closing() {
    let config = Config{
        prefix: String::from("<pre>"),
        suffix: String::from("</pre>"),
        keep_prefix: false,
        keep_suffix: true,
        trim: false
    };
    let src = String::from("<pre>1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>");
    let res = crate::extract_data_extended(&src, config);
    assert_eq!(
        res,
        vec![
            "1<pre>2<pre>33</pre>2</pre><pre>44</pre>1</pre>",
            "2<pre>33</pre>2</pre>",
            "33</pre>",
            "44</pre>"
        ]
    );
}