use markup::DynRender;

use crate::{models::posts::Post, utils::used_tech_tags};

use super::Layout;

#[inline(always)]
pub fn render<'a>(recent_posts: &Vec<Post>) -> Layout<'a, DynRender<'a>> {
  let recent_posts = recent_posts.iter().map(|post| {
    markup::new! {
      li {
        a[class="text-lg underline", href={format!("/blog/{}", post.post_url)}] {
          { format!("{} ({})", post.title, post.format_date()) }
        }
      }
      
    }.to_string()
  }).collect::<Vec<_>>().join("");

  let used_tech = used_tech_tags();

  Layout {
    location: "/",
    title: "Home",
    body: markup::new! {
      h1[class="text-4xl font-bold text-center mt-8"] { 
        "Chris Miller" 
      }
      p[class="text-center italic"] {
        "Computer Engineer / Maker"
      }
      p[class="my-2"] {
        "I'm a software engineer interested in distributed systems and making technology whimsical. "
        "I am currently based out of Seattle and work for Microsoft as a R&D tech lead. "
        "I love to experiment with techology and am a serial project starter. "
        "Follow my "
        a[href="/posts", class="underline"] {
          "blog"
        }
        " for semi-annual rambles."
      }
      div[class="text-center"] {
        a[href="https://github.com/ChristopherJMiller", class="underline mx-1"] {
          "Github"
        }
        a[href="https://www.linkedin.com/in/christopherjmill", class="underline mx-1"] {
          "LinkedIn"
        }
        a[href="https://void.lgbt/alumux", class="underline mx-1"] {
          "Fediverse"
        }
      }
      div {
        div[class="my-6"] {
          h1 [class="text-2xl mb-2"] {
            "Recent Blog Posts"
          }
          ul {
            @markup::raw(&recent_posts)
          }
        }
      }
      div {
        h1[class="text-2xl mb-2"] {
          "Tech I Use"
        }

        div[class="flex flex-wrap gap-2 underline"] {
          @markup::raw(&used_tech)
        }
      }
    }
  }
}