use std::fs;
use std::io::Write;
use std::path::PathBuf;
use parser;

pub fn post(post: &parser::Post) -> () {
    let path: PathBuf = [
        "public.nosync",
        &post.category,
        &post.pagename,
        "index.html",
    ].iter()
        .collect();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    write!(
        fs::File::create(path).unwrap(),
        r#"<!DOCTYPE html>
<html lang="cmn-Hans" manifest="/mono.appcache">
<head>
<meta charset="UTF-8">
<title>{title}</title>
<meta name="author" content="Daniel Zeng">
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">
<link rel="stylesheet" type="text/css" href="/mono.css">
<link rel="icon" type="image/png" href="/favicon.png">
<link rel="alternate" type="application/atom+xml" title="RSS" href="/atom.xml">
</head>
<body>
<header>
<a href="/"><h1>DarkNode</h1><h2>Life, the Universe and Everything</h2></a>
</header>
<article>
<h1>{title}</h1>
{data}
</article>
<footer>
<p>&copy;&nbsp;2014-2017&nbsp;<a href="/about/">Daniel Zeng</a>&nbsp;</p>
<p><a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">CC BY-NC-SA 4.0</a></p>
</footer>
</body>
</html>"#,
        title = post.title,
        data = post.data
    ).unwrap()
}
