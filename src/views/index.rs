use markup::DynRender;

use super::Layout;
use crate::models::posts::Post;

#[inline(always)]
pub fn render<'a>(recent_post: &'a Post) -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/",
    title: "Home",
    body: markup::new! {
      h1[class="text-4xl font-bold text-center mt-8"] {
        "Chris Miller"
      }
      p[class="text-center italic"] {
        "Computer Engineer / UX Designer / Maker"
      }
      div[class="text-center"] {
        a[href="https://github.com/ChristopherJMiller", class="underline mx-1"] {
          "Github"
        }
        a[href="https://www.linkedin.com/in/christopherjmill", class="underline mx-1"] {
          "LinkedIn"
        }
        div[class="my-6"] {
          h1 [class="text-2xl"] {
            "Most Recent Blog Post"
          }
          a[class="text-xl font-semibold underline", href={format!("/blog/{}", &recent_post.post_url)}] {
            { &recent_post.title }
          }
        }
      }
    },
  }
}
