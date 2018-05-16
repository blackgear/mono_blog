//! Render for Vec<Blog> collected by Parser as website
//!
//! Generate post, category index, main index and assets pages.
//!
//! # Example
//!
//! ```
//! use render::Site;
//!
//! Site::new(blog).render();
//! ```
use parser::{Blog, Post};
use rayon::prelude::*;
use rayon::scope;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const PAGESIZE: usize = 7;
const DESTROOT: &str = "public";

impl Ord for Post {
    fn cmp(&self, other: &Post) -> Ordering {
        self.modified.cmp(&other.modified)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Post) -> Option<Ordering> {
        Some(self.modified.cmp(&other.modified))
    }
}

fn create(path: PathBuf) -> BufWriter<File> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).unwrap();
    }
    BufWriter::new(fs::File::create(path).unwrap())
}

pub struct Site {
    blog: Vec<Post>,
}

impl Site {
    pub fn new(blog: Blog) -> Site {
        timer!("Parser");
        Site {
            blog: blog.collect(),
        }
    }

    pub fn render(&self) {
        timer!("Render");
        if self.blog.is_empty() {
            return;
        };
        scope(|s| {
            s.spawn(|_| self.render_post());
            s.spawn(|_| self.render_menu());
            s.spawn(|_| self.render_main());
            s.spawn(|_| self.render_feed());
            s.spawn(|_| self.render_site());
            s.spawn(|_| self.render_misc());
        })
    }

    fn render_post(&self) {
        self.blog.par_iter().for_each(|post| {
            let path = [DESTROOT, &post.category, &post.pagename, "index.html"]
                .iter()
                .collect();
            let mut w = create(path);
            wite!(
                w,
                "<!DOCTYPE html>\n"
                "<html lang=\"cmn-Hans\">\n"
                "<head>\n"
                "<meta charset=\"UTF-8\">\n"
                "<title>"(post.title)"</title>\n"
                "<meta name=\"author\" content=\"Daniel Zeng\">\n"
                "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no\">\n"
                "<link rel=\"stylesheet\" type=\"text/css\" href=\"/mono.css\">\n"
                "<link rel=\"icon\" type=\"image/png\" href=\"/favicon.png\">\n"
                "<link rel=\"alternate\" type=\"application/atom+xml\" title=\"RSS\" href=\"/atom.xml\">\n"
                "</head>\n"
                "<body>\n"
                "<header>\n"
                "<a href=\"/\"><h1>DarkNode</h1><h2>Life, the Universe and Everything</h2></a>\n"
                "</header>\n"
                "<article>\n"
                "<h1>"(post.title)"</h1>\n"
                "<pre><code>本文发表于：<time datetime=\""(post.released)"\">"(&post.released[0..10])"</time>\n"
                "最后修改于：<time datetime=\""(post.modified)"\">"(&post.modified[0..10])"</time>\n"
                if !post.category.is_empty() {
                    "分类：<a href=\"/"(post.category)"/\">"(post.category)"</a>\n"
                }
                "合计信息量："{((post.data.chars().count() as f64) / 1024.0):.2}"kb</code></pre>\n"
                (post.data)
                "</article>\n"
                "<footer>\n"
                "<p>&copy;&nbsp;2014-2018&nbsp;<a href=\"/about/\">Daniel Zeng</a>&nbsp;</p>\n"
                "<p><a href=\"https://creativecommons.org/licenses/by-nc-sa/4.0/\">CC BY-NC-SA 4.0</a></p>\n"
                "</footer>\n"
                "</body>\n"
                "</html>\n"
            ).unwrap();
        })
    }

    fn render_menu(&self) {
        let categories: HashSet<&str> = self.blog
            .iter()
            .filter(|x| !x.category.is_empty())
            .map(|x| x.category.as_ref())
            .collect();

        categories.into_par_iter().for_each(|category| {
            let path = [DESTROOT, category, "index.html"].iter().collect();
            let mut w = create(path);
            wite!(
                w,
                "<!DOCTYPE html>\n"
                "<html lang=\"cmn-Hans\">\n"
                "<head>\n"
                "<meta charset=\"UTF-8\">\n"
                "<title>DarkNode</title>\n"
                "<meta name=\"author\" content=\"Daniel Zeng\">\n"
                "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no\">\n"
                "<link rel=\"stylesheet\" type=\"text/css\" href=\"/mono.css\">\n"
                "<link rel=\"icon\" type=\"image/png\" href=\"/favicon.png\">\n"
                "<link rel=\"alternate\" type=\"application/atom+xml\" title=\"RSS\" href=\"/atom.xml\">\n"
                "</head>\n"
                "<body>\n"
                "<header>\n"
                "<a href=\"/\"><h1>DarkNode</h1><h2>Life, the Universe and Everything</h2></a>\n"
                "</header>\n"
                "<article>\n"
                "<nav>分类 - "(category)"</nav>"
                for post in self.blog.iter().filter(|x| x.category == category) {
                    "<section>\n"
                    "<a href=\"/"(post.category)"/"(post.pagename)"/\">"
                    "<h1>"(post.title)"</h1><time datetime=\""(post.released)"\">"(&post.released[0..10])"</time>"
                    "</a>\n"
                    "</section>\n"
                }
                "</article>\n"
                "<footer>\n"
                "<p>&copy;&nbsp;2014-2018&nbsp;<a href=\"/about/\">Daniel Zeng</a>&nbsp;</p>\n"
                "<p><a href=\"https://creativecommons.org/licenses/by-nc-sa/4.0/\">CC BY-NC-SA 4.0</a></p>\n"
                "</footer>\n"
                "</body>\n"
                "</html>\n"
            ).unwrap();
        })
    }

    fn render_main(&self) {
        let size = self.blog
            .iter()
            .filter(|post| !post.category.is_empty())
            .count();

        (1..size / PAGESIZE + 2).into_par_iter().for_each(|pid| {
            let path = if pid == 1 {
                [DESTROOT, "index.html"].iter().collect()
            } else {
                [DESTROOT, "page", &format!("{}", pid), "index.html"]
                    .iter()
                    .collect()
            };
            let mut w = create(path);
            wite!(
                w,
                "<!DOCTYPE html>\n"
                "<html lang=\"cmn-Hans\">\n"
                "<head>\n"
                "<meta charset=\"UTF-8\">\n"
                "<title>DarkNode</title>\n"
                "<meta name=\"author\" content=\"Daniel Zeng\">\n"
                "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no\">\n"
                "<link rel=\"stylesheet\" type=\"text/css\" href=\"/mono.css\">\n"
                "<link rel=\"icon\" type=\"image/png\" href=\"/favicon.png\">\n"
                "<link rel=\"alternate\" type=\"application/atom+xml\" title=\"RSS\" href=\"/atom.xml\">\n"
                "</head>\n"
                "<body>\n"
                "<header>\n"
                "<a href=\"/\"><h1>DarkNode</h1><h2>Life, the Universe and Everything</h2></a>\n"
                "</header>\n"
                "<article>\n"
                "<nav>索引 - P"(pid)
                if pid * PAGESIZE < size {
                    "<a href=\"/page/"(pid + 1)"/\">下页</a>"
                }
                if pid == 2 {
                    "<a href=\"/\">上页</a>"
                }
                if pid > 2 {
                    "<a href=\"/page/"(pid - 1)"/\">上页</a>"
                }
                "</nav>\n"
                for post in self.blog
                    .iter()
                    .filter(|post| !post.category.is_empty())
                    .skip((pid - 1) * PAGESIZE)
                    .take(PAGESIZE) {
                    "<section>\n"
                    "<a href=\"/"(post.category)"/"(post.pagename)"/\">"
                    "<h1>"(post.title)"</h1><time datetime=\""(post.released)"\">"(&post.released[0..10])"</time>"
                    "</a>\n"
                    "</section>\n"
                }
                "</article>\n"
                "<footer>\n"
                "<p>&copy;&nbsp;2014-2018&nbsp;<a href=\"/about/\">Daniel Zeng</a>&nbsp;</p>\n"
                "<p><a href=\"https://creativecommons.org/licenses/by-nc-sa/4.0/\">CC BY-NC-SA 4.0</a></p>\n"
                "</footer>\n"
                "</body>\n"
                "</html>\n"
            ).unwrap();
        })
    }

    fn render_feed(&self) {
        let path = [DESTROOT, "atom.xml"].iter().collect();
        let mut w = create(path);
        let mut posts: BinaryHeap<&Post> = self.blog.iter().collect();
        wite!(
            w,
            "<feed xmlns=\"http://www.w3.org/2005/Atom\">\n"
            "<title>DarkNode</title>\n"
            "<subtitle>Life, the Universe and Everything</subtitle>\n"
            "<link href=\"/atom.xml\" rel=\"self\"/>\n"
            "<link href=\"https://darknode.in/\"/>\n"
            "<updated>"(posts.peek().map_or("", |post| &post.modified))"</updated>\n"
            "<id>https://darknode.in/</id>\n"
            "<author>\n"
            "<name>Daniel Zeng</name>\n"
            "</author>\n"
            for _ in 0..3 {
                if let Some(post) = posts.pop() {
                    "<entry>\n"
                    "<title>"(post.title)"</title>\n"
                    "<link href=\"https://darknode.in/"(post.category)"/"(post.pagename)"/\"/>\n"
                    "<id>https://darknode.in/"(post.category)"/"(post.pagename)"/</id>\n"
                    "<published>"(post.released)"</published>\n"
                    "<updated>"(post.modified)"</updated>\n"
                    "<content type=\"html\">\n"
                    "<![CDATA[\n"
                    (post.data)
                    "]]>\n"
                    "</content>\n"
                    "</entry>\n"
                }
            }
            "</feed>\n"
        ).unwrap();
    }

    fn render_site(&self) {
        let path = [DESTROOT, "robots.txt"].iter().collect();
        let mut w = create(path);
        wite!(
            w,
            "User-agent: *\n"
            "Allow: /\n"
            "Sitemap: https://darknode.in/sitemap.xml\n"
        ).unwrap();

        let path = [DESTROOT, "sitemap.xml"].iter().collect();
        let mut w = create(path);
        wite!(
            w,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
            "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n"
            for pid in 1..self.blog.len() / PAGESIZE + 2 {
                "<url>\n"
                if pid == 1 {
                    "<loc>https://darknode.in/\n"
                } else {
                    "<loc>https://darknode.in/page/"(pid)"/\n"
                }
                "<lastmod>"
                (self.blog.iter().take(pid * PAGESIZE).max().map_or("", |post| &post.modified))
                "</lastmod>\n"
                "<changefreq>weekly</changefreq>\n"
                "<priority>1.0</priority>\n"
                "</url>\n"
            }
            for post in &self.blog {
                "<url>\n"
                if !post.category.is_empty() {
                    "<loc>https://darknode.in/"(post.category)"/"(post.pagename)"/</loc>\n"
                } else {
                    "<loc>https://darknode.in/"(post.pagename)"/</loc>\n"
                }
                "<lastmod>"(post.modified)"</lastmod>\n"
                "<changefreq>monthly</changefreq>\n"
                if !post.category.is_empty() {
                    "<priority>0.8</priority>\n"
                } else {
                    "<priority>0.5</priority>\n"
                }
                "</url>\n"
            }
            "</urlset>\n"
            ).unwrap();
    }

    fn render_misc(&self) {
        let path = [DESTROOT, "mono.css"].iter().collect();
        let mut w = create(path);
        w.write_all(include_bytes!("mono.css")).unwrap();

        let path = [DESTROOT, "favicon.png"].iter().collect();
        let mut w = create(path);
        w.write_all(include_bytes!("favicon.png")).unwrap();
    }
}
